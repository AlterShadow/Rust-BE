use crate::logger::get_blockchain_logger;
use crate::{
    EitherTransport, EthereumRpcConnectionGuard, EthereumRpcConnectionPool, MultiChainAddressTable,
    CONFIRMATIONS,
};
use eyre::{bail, ContextCompat};
use gen::model::EnumBlockChain;
use lib::log::DynLogger;
use serde_json::Value;
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::hash::Hash;
use std::path::Path;
use std::{collections::HashMap, time};
use tracing::warn;
use web3::api::{Accounts, Eth, Namespace};
use web3::contract::deploy::Error;
use web3::contract::tokens::Tokenize;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, TransactionParameters, TransactionReceipt, TransactionRequest};
use web3::{ethabi, Transport};

use crate::utils::wait_for_confirmations;

/// A configuration builder for contract deployment.
#[derive(Debug)]
pub struct ContractDeployer<T: Transport> {
    pub(crate) eth: Eth<T>,
    pub(crate) abi: ethabi::Contract,
    pub(crate) options: Options,
    pub(crate) max_retries: usize,
    pub(crate) confirmations: u64,
    pub(crate) poll_interval: time::Duration,
    pub(crate) linker: HashMap<String, Address>,
    pub(crate) code: Option<String>,
}

impl<T: Transport> ContractDeployer<T> {
    pub fn new(eth: Eth<T>, abi_json: Value) -> ethabi::Result<Self> {
        let abi = serde_json::from_value(abi_json)?;
        Ok(Self {
            eth,
            abi,
            options: Options::default(),
            max_retries: 1,
            confirmations: CONFIRMATIONS, // not enough for ethereum. should be 14
            poll_interval: time::Duration::from_secs(3),
            linker: HashMap::default(),
            code: None,
        })
    }
    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }
    /// Number of confirmations required after code deployment.
    pub fn max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Deployment transaction options.
    pub fn options(mut self, options: Options) -> Self {
        self.options = options;
        self
    }

    /// Confirmations poll interval.
    pub fn poll_interval(mut self, interval: time::Duration) -> Self {
        self.poll_interval = interval;
        self
    }

    /// Execute deployment passing code and constructor parameters.
    ///
    /// Unlike the above `sign_and_execute`, this method allows the
    /// caller to pass in a private key to sign the transaction with
    /// and therefore allows deploying from an account that the
    /// ethereum node doesn't need to know the private key for.
    ///
    /// An optional `chain_id` parameter can be passed to provide
    /// replay protection for transaction signatures. Passing `None`
    /// would create a transaction WITHOUT replay protection and
    /// should be avoided.
    /// You can obtain `chain_id` of the network you are connected
    /// to using `web3.eth().chain_id()` method.
    pub async fn sign_with_key_and_execute<P, K>(
        &self,
        params: P,
        signer: K,
        logger: DynLogger,
    ) -> eyre::Result<Contract<T>>
    where
        P: Tokenize,
        K: Key + Clone,
    {
        let transport = self.eth.transport().clone();
        let poll_interval = self.poll_interval;
        let chain_id = Some(self.eth.chain_id().await?.as_u64());
        let eth = self.eth.clone();
        let gas = self.options.gas;
        let confirmations = self.confirmations;
        let logger2 = logger.clone();
        let contract = self
            .do_execute(
                self.code.as_deref().context("Code is not provided")?,
                params,
                signer.address(),
                move |tx| {
                    let transport = transport.clone();
                    let signer = signer.clone();
                    let eth = eth.clone();
                    let logger = logger2.clone();
                    async move {
                        let tx = TransactionParameters {
                            nonce: tx.nonce,
                            to: tx.to,
                            gas: gas.expect("No gas set"),
                            gas_price: tx.gas_price,
                            value: tx.value.unwrap_or_else(|| 0.into()),
                            data: tx.data.expect(
                                "Tried to deploy a contract but transaction data wasn't set",
                            ),
                            chain_id,
                            transaction_type: tx.transaction_type,
                            access_list: tx.access_list,
                            max_fee_per_gas: tx.max_fee_per_gas,
                            max_priority_fee_per_gas: tx.max_priority_fee_per_gas,
                        };
                        logger.log(format!("Signing transaction"));
                        let signed_tx = Accounts::new(transport)
                            .sign_transaction(tx, signer)
                            .await?;
                        logger.log(format!(
                            "Sending transaction: {:?}",
                            signed_tx.transaction_hash
                        ));
                        // TODO: buggy here
                        let tx_hash = eth.send_raw_transaction(signed_tx.raw_transaction).await?;
                        get_blockchain_logger().log(
                            format!("Transaction sent, waiting for confirmations",),
                            tx_hash,
                        );
                        logger.log(format!(
                            "Transaction sent, waiting for confirmations: {:?}",
                            tx_hash
                        ));
                        match wait_for_confirmations(
                            &eth,
                            tx_hash,
                            poll_interval,
                            10,
                            confirmations as _,
                        )
                        .await
                        {
                            Ok(receipt) => Ok(receipt),
                            Err(err) => {
                                // TODO: implement deployment retry if error is RpcError, TransactionNotFoundAfterConfirmations, or TransactionRevertedAfterConfirmations
                                bail!(
                                    "failed to wait for confirmations on contract deployment: {:?}",
                                    err
                                )
                            }
                        }
                    }
                },
                logger,
            )
            .await?;
        Ok(contract)
    }

    async fn do_execute<P, V, Ft>(
        &self,
        code: V,
        params: P,
        from: Address,
        send: impl Fn(TransactionRequest) -> Ft,
        logger: DynLogger,
    ) -> eyre::Result<Contract<T>>
    where
        P: Tokenize,
        V: AsRef<str>,
        Ft: Future<Output = eyre::Result<TransactionReceipt>>,
    {
        let options = &self.options;
        let eth = &self.eth;
        let abi = &self.abi;

        let mut code_hex = code.as_ref().to_string();

        for (lib, address) in &self.linker {
            if lib.len() > 38 {
                return Err(Error::Abi(ethabi::Error::InvalidName(
                    "The library name should be under 39 characters.".into(),
                ))
                .into());
            }
            let replace = format!("__{:_<38}", lib); // This makes the required width 38 characters and will pad with `_` to match it.
            let address: String = hex::encode(address);
            code_hex = code_hex.replacen(&replace, &address, 1);
        }
        code_hex = code_hex.replace("\"", "").replace("0x", ""); // This is to fix truffle + serde_json redundant `"` and `0x`
        let code = hex::decode(&code_hex)
            .map_err(|e| ethabi::Error::InvalidName(format!("hex decode error: {}", e)))?;

        let params = params.into_tokens();
        let data = match (abi.constructor(), params.is_empty()) {
            (None, false) => {
                return Err(Error::Abi(ethabi::Error::InvalidName(
                    "Constructor is not defined in the ABI.".into(),
                ))
                .into());
            }
            (None, true) => code,
            (Some(constructor), _) => constructor.encode_input(code, &params)?,
        };

        let tx = TransactionRequest {
            from,
            to: None,
            gas: options.gas,
            gas_price: options.gas_price,
            value: options.value,
            nonce: options.nonce,
            data: Some(data.into()),
            condition: options.condition.clone(),
            transaction_type: options.transaction_type,
            access_list: options.access_list.clone(),
            max_fee_per_gas: options.max_fee_per_gas,
            max_priority_fee_per_gas: options.max_priority_fee_per_gas,
        };
        for _ in 0..self.max_retries {
            logger.log(format!("Deploying contract with nonce {:?}", tx.nonce));
            let receipt = send(tx.clone()).await;
            match receipt {
                Ok(receipt) => {
                    logger.log(format!("transaction hash {}", receipt.transaction_hash));
                    return match receipt.status {
                        Some(status) if status == 0.into() => {
                            Err(Error::ContractDeploymentFailure(receipt.transaction_hash).into())
                        }
                        // If the `status` field is not present we use the presence of `contract_address` to
                        // determine if deployment was successfull.
                        _ => match receipt.contract_address {
                            Some(address) => Ok(Contract::new(eth.clone(), address, abi.clone())),
                            None => {
                                Err(Error::ContractDeploymentFailure(receipt.transaction_hash)
                                    .into())
                            }
                        },
                    };
                }
                Err(err) => {
                    warn!("Failed to deploy contract, retrying: {:?}", err);
                }
            }
        }
        bail!("Max retries exceeded, failed to deploy contract")
    }
}

pub fn read_abi_from_solc_output(path: &Path) -> eyre::Result<Value> {
    let json = std::fs::read(path)?;
    let json: Value = serde_json::from_slice(&json)?;
    let abi = json.get("abi").context("No abi")?;
    Ok(abi.to_owned())
}

pub struct AbstractContract<ENUM = ()> {
    pub name: String,
    pub abi: ethabi::Contract,
    pub contract_addresses: MultiChainAddressTable<ENUM>,
}
impl<ENUM> Debug for AbstractContract<ENUM> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AbstractContract")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}
impl<ENUM: Copy + Eq + Hash + Debug> AbstractContract<ENUM> {
    pub fn to_concrete_contract(
        &self,
        eth: Eth<EitherTransport>,
        address: Address,
    ) -> Contract<EitherTransport> {
        Contract::new(eth, address, self.abi.clone())
    }
    pub async fn get_with_web3(
        &self,
        pool: &EthereumRpcConnectionPool,
        blockchain: EnumBlockChain,
        enum_: ENUM,
    ) -> eyre::Result<(Contract<EitherTransport>, EthereumRpcConnectionGuard)> {
        let web3 = pool.get(blockchain).await?;
        let address = self
            .contract_addresses
            .get(blockchain, enum_)
            .with_context(|| {
                format!(
                    "Could not find contract address for {} on {}",
                    self.name, blockchain
                )
            })?;
        let contract = self.to_concrete_contract(web3.eth(), address);
        Ok((contract, web3))
    }
    pub async fn get(
        &self,
        pool: &EthereumRpcConnectionPool,
        blockchain: EnumBlockChain,
        enum_: ENUM,
    ) -> eyre::Result<Contract<EitherTransport>> {
        let (contract, _guard) = self.get_with_web3(pool, blockchain, enum_).await?;
        Ok(contract)
    }
}
