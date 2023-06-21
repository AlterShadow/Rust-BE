use crate::contract::AbstractContract;
use crate::utils::wait_for_confirmations;
use crate::{
    EitherTransport, EthereumRpcConnection, EthereumRpcConnectionPool, MultiChainAddressTable,
};
use eyre::*;
use gen::model::EnumBlockChain;
use std::fmt::{Debug, Formatter};
use std::time::Duration;
use tracing::warn;
use web3::api::Web3;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};

pub const ERC20_ABI: &'static str = include_str!("erc20.abi.json");

pub struct AbstractErc20Token(AbstractContract<()>);
impl AbstractErc20Token {
    pub fn new(name: String, table: MultiChainAddressTable<()>) -> Self {
        Self(AbstractContract {
            name,
            abi: build_erc_20().unwrap(),
            contract_addresses: table,
        })
    }
    pub async fn get(
        &self,
        pool: &EthereumRpcConnectionPool,
        blockchain: EnumBlockChain,
    ) -> Result<Erc20Token> {
        let contract = self.0.get(pool, blockchain, ()).await?;
        Ok(Erc20Token {
            address: contract.address(),
            contract,
        })
    }
}

#[derive(Clone)]
pub struct Erc20Token {
    pub address: Address,
    pub contract: Contract<EitherTransport>,
}

impl Erc20Token {
    pub fn new(client: Web3<EitherTransport>, address: Address) -> Result<Self> {
        Ok(Self {
            address,
            contract: Contract::new(client.eth(), address, build_erc_20()?),
        })
    }

    pub fn new_with_abi(
        client: Web3<EitherTransport>,
        address: Address,
        abi: web3::ethabi::Contract,
    ) -> Result<Self> {
        Ok(Self {
            address,
            contract: Contract::new(client.eth(), address, abi),
        })
    }

    pub async fn symbol(&self) -> Result<String> {
        Ok(self
            .contract
            .query("symbol", (), None, Options::default(), None)
            .await?)
    }

    pub async fn decimals(&self) -> Result<U256> {
        Ok(self
            .contract
            .query("decimals", (), None, Options::default(), None)
            .await?)
    }

    pub async fn mint(
        &self,
        conn: &EthereumRpcConnection,
        secret: impl Key,
        to: Address,
        amount: U256,
    ) -> Result<H256> {
        let estimated_gas = self
            .contract
            .estimate_gas("mint", (to, amount), secret.address(), Options::default())
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        Ok(self
            .contract
            .signed_call(
                "mint",
                (to, amount),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                secret,
            )
            .await?)
    }

    pub async fn burn(
        &self,
        conn: &EthereumRpcConnection,
        secret: impl Key,
        from: Address,
        amount: U256,
    ) -> Result<H256> {
        let estimated_gas = self
            .contract
            .estimate_gas("burn", (from, amount), secret.address(), Options::default())
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        Ok(self
            .contract
            .signed_call(
                "burn",
                (from, amount),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                secret,
            )
            .await?)
    }

    pub async fn transfer(
        &self,
        conn: &EthereumRpcConnection,
        secret: impl Key,
        to: Address,
        amount: U256,
    ) -> Result<H256> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                "transfer",
                (to, amount),
                secret.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        Ok(self
            .contract
            .signed_call(
                "transfer",
                (to, amount),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                secret,
            )
            .await?)
    }

    pub async fn transfer_from(
        &self,
        conn: &EthereumRpcConnection,
        secret: impl Key,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<H256> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                "transferFrom",
                (from, to, amount),
                secret.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        Ok(self
            .contract
            .signed_call(
                "transferFrom",
                (from, to, amount),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                secret,
            )
            .await?)
    }

    pub async fn approve(
        &self,
        conn: &EthereumRpcConnection,
        secret: impl Key,
        spender: Address,
        amount: U256,
    ) -> Result<H256> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                "approve",
                (spender, amount),
                secret.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;

        Ok(self
            .contract
            .signed_call(
                "approve",
                (spender, amount),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                secret,
            )
            .await?)
    }

    pub async fn balance_of(&self, owner: Address) -> Result<U256> {
        Ok(self
            .contract
            .query("balanceOf", owner, None, Options::default(), None)
            .await?)
    }

    pub async fn allowance(&self, owner: Address, spender: Address) -> Result<U256> {
        Ok(self
            .contract
            .query(
                "allowance",
                (owner, spender),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn total_supply(&self) -> Result<U256> {
        Ok(self
            .contract
            .query("totalSupply", (), None, Options::default(), None)
            .await?)
    }
}

impl Debug for Erc20Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ERC20Token")
            .field("address", &self.address)
            .finish()
    }
}

pub fn build_erc_20() -> Result<web3::ethabi::Contract> {
    Ok(web3::ethabi::Contract::load(ERC20_ABI.as_bytes())
        .context("failed to parse contract ABI")?)
}

pub async fn approve_and_ensure_success(
    contract: Erc20Token,
    conn: &EthereumRpcConnection,
    confirmations: u64,
    max_retry: usize,
    wait_timeout: Duration,
    signer: impl Key + Clone,
    spender: Address,
    amount: U256,
) -> Result<H256> {
    for retries in 0..max_retry {
        /* publish transaction */
        let tx_hash = contract
            .approve(&conn, signer.clone(), spender, amount)
            .await?;
        match wait_for_confirmations(&conn.eth(), tx_hash, wait_timeout, max_retry, confirmations)
            .await
        {
            Ok(_ok) => {
                /* transaction is confirmed, consider it canonical */
                return Ok(tx_hash);
            }
            Err(err) => {
                warn!(
                    "Transaction {:?} failed to confirm after {:?} retries: {:?}",
                    tx_hash, retries, err
                );
            }
        }
    }
    bail!(
        "Transaction failed to confirm after {:?} retries",
        max_retry
    )
}
