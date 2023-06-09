use std::fmt::{Debug, Formatter};
use std::time::Duration;

use eyre::*;
use tokio::time::sleep;
use web3::api::Web3;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};

use crate::contract::AbstractContract;
use crate::{
    wait_for_confirmations_simple, EitherTransport, EthereumRpcConnection,
    EthereumRpcConnectionPool, MultiChainAddressTable, TransactionFetcher, TxStatus,
};
use gen::model::EnumBlockChain;

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

    pub async fn mint(&self, secret: impl Key, to: Address, amount: U256) -> Result<H256> {
        Ok(self
            .contract
            .signed_call("mint", (to, amount), Options::default(), secret)
            .await?)
    }

    pub async fn burn(&self, secret: impl Key, from: Address, amount: U256) -> Result<H256> {
        Ok(self
            .contract
            .signed_call("burn", (from, amount), Options::default(), secret)
            .await?)
    }

    pub async fn transfer(&self, secret: impl Key, to: Address, amount: U256) -> Result<H256> {
        Ok(self
            .contract
            .signed_call("transfer", (to, amount), Options::default(), secret)
            .await?)
    }

    pub async fn transfer_from(
        &self,
        secret: impl Key,
        from: Address,
        to: Address,
        amount: U256,
    ) -> Result<H256> {
        Ok(self
            .contract
            .signed_call(
                "transferFrom",
                (from, to, amount),
                Options::default(),
                secret,
            )
            .await?)
    }

    pub async fn approve(&self, secret: impl Key, spender: Address, amount: U256) -> Result<H256> {
        Ok(self
            .contract
            .signed_call("approve", (spender, amount), Options::default(), secret)
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
    /* publish transaction */
    let mut tx_hash = contract.approve(signer.clone(), spender, amount).await?;
    let mut retries: usize = 0;
    while retries < max_retry {
        /* wait for transaction receipt */
        /* after it has a receipt, it was included in a block */
        let tx_receipt =
            wait_for_confirmations_simple(&conn.eth(), tx_hash, wait_timeout, max_retry).await?;

        /* get receipt block number */
        let tx_block_number = tx_receipt
            .block_number
            .ok_or_else(|| eyre!("transaction has receipt but was not included in a block"))?
            .as_u64();
        let mut current_block_number = conn.eth().block_number().await?.as_u64();

        while current_block_number - tx_block_number < confirmations {
            /* wait for confirmations */
            /* more confirmations = greater probability that the transaction status is canonical */
            current_block_number = conn.eth().block_number().await?.as_u64();
            sleep(wait_timeout).await;
        }

        /* after confirmations, find out transaction status */
        let mut tx = TransactionFetcher::new(tx_hash);
        tx.update(&conn).await?;

        match tx.get_status() {
            TxStatus::Successful => {
                /* transaction is successful after confirmations, consider it canonical*/
                break;
            }
            TxStatus::Pending => {
                /* TODO: check if this is even possible */
                /* transaction had a receipt (was included in a block) but has somehow returned to the mempool */
                /* wait for the new receipt */
                retries += 1;
                continue;
            }
            TxStatus::Reverted | TxStatus::NotFound => {
                /* transaction is reverted or doesn't exist after confirmations, try again */
                retries += 1;
                tx_hash = contract.approve(signer.clone(), spender, amount).await?;
            }
            _ => continue,
        }
    }
    Ok(tx_hash)
}
