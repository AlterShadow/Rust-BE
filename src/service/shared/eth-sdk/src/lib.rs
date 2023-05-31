use crate::utils::{wait_for_confirmations_simple, wei_to_eth};
use eyre::*;
use std::fmt::{Debug, Formatter};
use std::time::Duration;
use tracing::info;
use web3::signing::Key;
use web3::types::{Address, TransactionParameters, TransactionRequest, H160, H256, U256};
use web3::Web3;

mod calldata;
mod conn;
pub mod contract;
mod contract_wrappers;
pub mod erc20;
// #[cfg(test)]
mod dex;
pub mod mock_erc20;
mod pool;
pub mod signer;
mod stablecoins;
mod tx;
pub mod utils;

pub use calldata::*;
pub use conn::*;
pub use contract_wrappers::*;
pub use dex::*;
pub use pool::*;
pub use stablecoins::*;
pub use tx::*;

#[derive(Clone)]
pub struct EthereumToken {
    pub client: Web3<EitherTransport>,
}
impl EthereumToken {
    pub fn new2(web3: Web3<EitherTransport>) -> Self {
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
    pub token: StableCoin,
    pub amount: U256,
    pub recipient: H160,
    pub owner: H160,
}

pub const ANVIL_PRIV_KEY_1: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
pub const ANVIL_PRIV_KEY_2: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
pub const ANVIL_PRIV_KEY_3: &str =
    "5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a";
pub const ANVIL_PRIV_KEY_4: &str =
    "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6";
