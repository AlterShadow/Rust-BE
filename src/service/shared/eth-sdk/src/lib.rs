use crate::utils::{wait_for_confirmations_simple, wei_to_eth};
use eyre::*;
use std::fmt::{Debug, Formatter};
use std::time::Duration;
use tracing::info;
use web3::signing::Key;
use web3::types::{Address, TransactionParameters, TransactionRequest, H256, U256};
use web3::Web3;
mod calldata;
mod conn;
pub mod contract;
mod contract_wrappers;
mod pancake_swap;
// #[cfg(test)]
mod address_table;
mod calc;
mod coins;
mod dex;
pub mod dex_tracker;
mod escrow_addresses;
pub mod escrow_tracker;
pub mod evm;
pub mod logger;
mod pool;
pub mod signer;
mod strategy_pool_herald_addresses;
mod tx;
pub mod utils;

pub use address_table::*;
pub use calc::*;
pub use calldata::*;
pub use coins::*;
pub use conn::*;
pub use contract_wrappers::*;
pub use dex::*;
pub use escrow_addresses::*;
pub use pancake_swap::*;
pub use pool::*;
pub use strategy_pool_herald_addresses::*;
pub use tx::*;

#[derive(Clone)]
pub struct EthereumToken {
    pub client: Web3<EitherTransport>,
}
impl EthereumToken {
    pub fn new(web3: Web3<EitherTransport>) -> Self {
        Self { client: web3 }
    }
    pub async fn transfer(&self, key: impl Key, to: Address, amount: U256) -> Result<H256> {
        info!(
            "Transferring {} ETH from {:?} to {:?}",
            wei_to_eth(amount),
            key.address(),
            to
        );
        let gas_price = self.client.eth().gas_price().await?;
        let params = TransactionParameters {
            to: Some(to),
            gas: 21000.into(),
            gas_price: Some(gas_price),
            value: amount,
            data: "".into(),
            nonce: None,
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
            chain_id: None,
        };
        let signed_transaction = self.client.accounts().sign_transaction(params, key).await?;
        let tx = self
            .client
            .eth()
            .send_raw_transaction(signed_transaction.raw_transaction)
            .await?;
        wait_for_confirmations_simple(&self.client.eth(), tx, Duration::from_secs(3), 5).await?;
        Ok(tx)
    }
    pub async fn get_accounts(&self) -> Result<Vec<Address>> {
        let accounts = self.client.eth().accounts().await?;

        Ok(accounts)
    }
    pub async fn transfer_debug(&self, from: Address, to: Address, amount: f64) -> Result<String> {
        let amount = (amount * 1e18) as u64;
        let nonce = self.client.eth().transaction_count(from, None).await?;
        let gas_price = self.client.eth().gas_price().await?;
        let tx = TransactionRequest {
            from: from,
            nonce: Some(nonce),
            gas_price: Some(gas_price),
            to: Some(to),
            value: Some(amount.into()),
            ..Default::default()
        };
        let tx_hash = self.client.eth().send_transaction(tx).await?;
        Ok(format!("{:?}", tx_hash))
    }
}
impl Debug for EthereumToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EthereumToken").finish()
    }
}

// TODO: put it in proper module
#[derive(Clone, Debug)]
pub struct EscrowTransfer {
    pub token: String,
    pub token_address: Address,
    pub amount: U256,
    pub recipient: Address,
    pub owner: Address,
}

pub const ANVIL_PRIV_KEY_1: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
pub const ANVIL_PRIV_KEY_2: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
pub const ANVIL_PRIV_KEY_3: &str =
    "5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";
pub const ANVIL_PRIV_KEY_4: &str =
    "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6";
/* please send testnet native tokens to this account from time to time */
pub const DEV_ACCOUNT_PRIV_KEY: &str =
    "bc0846d716105203f84e0c841a63faa5d7b20addff1975b1554485b5a13a8061";
pub const DEV_ACCOUNT_ADDRESS: &str = "0x8A2D8E538E8544B77303E57950d526Da42D54af3";

// TODO: increase confirmations to 14 when we go to public
pub const CONFIRMATIONS: u64 = 1;
pub const MAX_RETRIES: u64 = 8;
pub const POLL_INTERVAL: Duration = Duration::from_secs(3);
