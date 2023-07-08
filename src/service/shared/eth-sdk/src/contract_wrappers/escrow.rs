use std::time::Duration;

use crate::contract::AbstractContract;
use crate::logger::get_blockchain_logger;
use crate::utils::wait_for_confirmations;
use crate::{
    deploy_contract, EitherTransport, EscrowAddresses, EthereumRpcConnection,
    EthereumRpcConnectionPool, MultiChainAddressTable,
};
use eyre::*;
use gen::model::EnumBlockChain;
use lib::log::DynLogger;
use lib::types::amount_to_display;
use tracing::info;
use web3::api::Eth;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::{ethabi, Transport, Web3};

const ESCROW_ABI_JSON: &str = include_str!("escrow.json");

pub struct AbstractEscrowContract(AbstractContract<()>);
impl AbstractEscrowContract {
    pub fn new(table: MultiChainAddressTable<()>) -> Self {
        let abi = ethabi::Contract::load(ESCROW_ABI_JSON.as_bytes()).unwrap();

        Self(AbstractContract {
            name: "Escrow".to_string(),
            abi,
            contract_addresses: table,
        })
    }
    pub fn new2(table: EscrowAddresses) -> Self {
        let abi = ethabi::Contract::load(ESCROW_ABI_JSON.as_bytes()).unwrap();

        Self(AbstractContract {
            name: "Escrow".to_string(),
            abi,
            contract_addresses: table.0,
        })
    }

    pub async fn get(
        &self,
        pool: &EthereumRpcConnectionPool,
        blockchain: EnumBlockChain,
    ) -> Result<EscrowContract<EitherTransport>> {
        let contract = self.0.get(pool, blockchain, ()).await?;
        Ok(EscrowContract { contract })
    }
}

#[derive(Debug, Clone)]
pub struct EscrowContract<T: Transport> {
    contract: Contract<T>,
}

impl<T: Transport> EscrowContract<T> {
    pub async fn deploy(w3: Web3<T>, key: impl Key + Clone) -> Result<Self> {
        let address = key.address();
        let contract = deploy_contract(w3, key, address, "Escrow", DynLogger::empty()).await?;
        Ok(Self { contract })
    }

    pub fn new(eth: Eth<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(eth, address, ESCROW_ABI_JSON.as_bytes())?;
        Ok(Self { contract })
    }

    pub fn address(&self) -> Address {
        self.contract.address()
    }

    pub async fn estimate_gas_transfer_token_to(
        &self,
        signer: impl Key,
        token_address: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<U256> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::TransferTokenTo.as_str(),
                (token_address, recipient, amount),
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(estimated_gas)
    }

    pub async fn transfer_token_to(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        token_address: Address,
        recipient: Address,
        amount: U256,
        logger: DynLogger,
    ) -> Result<H256> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::TransferTokenTo.as_str(),
                (token_address, recipient, amount),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        info!(
            "Transferring {:?} amount of token {:?} to recipient {:?} from escrow contract {:?} by {:?}",
            amount,
            token_address,
            recipient,
            self.address(),
            signer.address(),
        );
        logger.log(format!(
            "Transferring {} amount of token {:?} to recipient {:?} from escrow contract {:?} by {:?}",
            amount_to_display(amount),
            token_address,
            recipient,
            self.address(),
            signer.address(),
        ));
        let tx_hash = self
            .contract
            .signed_call(
                EscrowFunctions::TransferTokenTo.as_str(),
                (token_address, recipient, amount),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;
        get_blockchain_logger().log(
            format!(
                "Transfer {:?} to {:?} tx_hash {:?}",
                amount, recipient, tx_hash
            ),
            tx_hash,
        )?;
        Ok(tx_hash)
    }

    pub async fn transfer_ownership(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        new_owner: Address,
    ) -> Result<H256> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                EscrowFunctions::TransferOwnership.as_str(),
                new_owner,
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        info!(
            "Transferring ownership from {:?} to {:?} of escrow contract {:?} by {:?}",
            self.owner().await?,
            new_owner,
            self.address(),
            signer.address(),
        );

        Ok(self
            .contract
            .signed_call(
                EscrowFunctions::TransferOwnership.as_str(),
                new_owner,
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?)
    }

    pub async fn owner(&self) -> Result<Address> {
        Ok(self
            .contract
            .query(
                EscrowFunctions::Owner.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }
}

enum EscrowFunctions {
    TransferTokenTo,
    TransferOwnership,
    Owner,
}

impl EscrowFunctions {
    fn as_str(&self) -> &'static str {
        match self {
            Self::TransferTokenTo => "transferTokenTo",
            Self::TransferOwnership => "transferOwnership",
            Self::Owner => "owner",
        }
    }
}

pub async fn transfer_token_to_and_ensure_success(
    contract: EscrowContract<EitherTransport>,
    conn: &EthereumRpcConnection,
    confirmations: u64,
    max_retry: u64,
    poll_interval: Duration,
    signer: impl Key + Clone,
    token_address: Address,
    recipient: Address,
    amount: U256,
    logger: DynLogger,
) -> Result<H256> {
    let tx_hash = contract
        .transfer_token_to(
            &conn,
            signer.clone(),
            token_address,
            recipient,
            amount,
            logger,
        )
        .await?;
    wait_for_confirmations(
        &conn.eth(),
        tx_hash,
        poll_interval,
        max_retry,
        confirmations,
    )
    .await?;
    Ok(tx_hash)
}

pub async fn transfer_ownership_and_ensure_success(
    contract: EscrowContract<EitherTransport>,
    conn: &EthereumRpcConnection,
    confirmations: u64,
    max_retry: u64,
    wait_timeout: Duration,
    signer: impl Key + Clone,
    new_owner: Address,
) -> Result<H256> {
    /* publish transaction */
    let tx_hash = contract
        .transfer_ownership(&conn, signer.clone(), new_owner)
        .await?;
    wait_for_confirmations(&conn.eth(), tx_hash, wait_timeout, max_retry, confirmations).await?;
    Ok(tx_hash)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use once_cell::sync::Lazy;
    use tokio::sync::Mutex;

    use super::*;
    use crate::contract_wrappers::mock_erc20::deploy_mock_erc20;
    use crate::signer::Secp256k1SecretKey;
    use crate::{
        wait_for_confirmations_simple, EthereumRpcConnectionGuard, EthereumRpcConnectionPool,
        ANVIL_PRIV_KEY_1, ANVIL_PRIV_KEY_2,
    };
    use gen::model::EnumBlockChain;

    static TX_CONN: Lazy<Arc<Mutex<Option<EthereumRpcConnectionGuard>>>> =
        Lazy::new(|| Arc::new(Mutex::new(None)));

    async fn get_tx_conn() -> Result<Arc<Mutex<Option<EthereumRpcConnectionGuard>>>> {
        /* since tests are parallel and use a single key, ensure only one test publishes transactions at a time */
        /* to avoid "nonce too low" errors */
        let tx_conn_arc = TX_CONN.clone();
        let mut tx_conn = tx_conn_arc.lock().await;
        if tx_conn.is_none() {
            *tx_conn = Some(
                EthereumRpcConnectionPool::new()
                    .get(EnumBlockChain::LocalNet)
                    .await?,
            );
        }
        Ok(tx_conn_arc.clone())
    }

    #[tokio::test]
    async fn test_transfer_token_to() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let mock_erc20_a = deploy_mock_erc20(tx_conn.clone(), god_key.clone()).await?;
        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let escrow = EscrowContract::deploy(tx_conn.clone(), god_key.clone()).await?;

        let logger = DynLogger::new(Arc::new(move |msg| {
            println!("{}", msg);
        }));

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            mock_erc20_a
                .mint(&tx_conn, god_key.clone(), escrow.address(), U256::from(100))
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(escrow.address()).await?,
            U256::from(100)
        );
        assert_eq!(
            mock_erc20_a.balance_of(alice.address()).await?,
            U256::from(0)
        );

        wait_for_confirmations_simple(
            &tx_conn.clone().eth(),
            escrow
                .transfer_token_to(
                    &tx_conn,
                    god_key.clone(),
                    mock_erc20_a.address,
                    alice.address(),
                    U256::from(100),
                    logger.clone(),
                )
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(
            mock_erc20_a.balance_of(escrow.address()).await?,
            U256::from(0)
        );
        assert_eq!(
            mock_erc20_a.balance_of(alice.address()).await?,
            U256::from(100)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_owner() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let escrow = EscrowContract::deploy(tx_conn.clone(), god_key.clone()).await?;

        assert_eq!(escrow.owner().await?, god_key.address());
        Ok(())
    }

    #[tokio::test]
    async fn test_transfer_ownership() -> Result<()> {
        let tx_conn_wrapper = get_tx_conn().await?;
        let mut tx_conn_guard = tx_conn_wrapper.lock().await;
        let tx_conn = tx_conn_guard.as_mut().unwrap();

        let god_key = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_1)?;

        let alice = Secp256k1SecretKey::from_str(ANVIL_PRIV_KEY_2)?;

        let escrow = EscrowContract::deploy(tx_conn.clone(), god_key.clone()).await?;

        assert_eq!(escrow.owner().await?, god_key.address());

        wait_for_confirmations_simple(
            &tx_conn.eth(),
            escrow
                .transfer_ownership(&tx_conn, god_key.clone(), alice.address())
                .await?,
            Duration::from_millis(1),
            10,
        )
        .await?;

        assert_eq!(escrow.owner().await?, alice.address());
        Ok(())
    }
}
