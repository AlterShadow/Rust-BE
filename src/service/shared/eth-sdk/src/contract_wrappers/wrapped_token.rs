use eyre::*;

use lib::types::amount_to_display;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::{Transport, Web3};

use crate::logger::get_blockchain_logger;
use crate::EthereumRpcConnection;
use crate::RpcCallError;

const WRAPPED_ABI_JSON: &str = include_str!("weth.json");

#[derive(Debug, Clone)]
pub struct WrappedTokenContract<T: Transport> {
    contract: Contract<T>,
}

impl<T: Transport> WrappedTokenContract<T> {
    pub fn new(w3: Web3<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(w3.eth(), address, WRAPPED_ABI_JSON.as_bytes())?;
        Ok(Self { contract })
    }

    pub fn address(&self) -> Address {
        self.contract.address()
    }

    pub async fn wrap(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        amount: U256,
    ) -> Result<H256, RpcCallError> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                WrappedTokenFunctions::Wrap.as_str(),
                (),
                signer.address(),
                Options::with(|options| {
                    options.value = Some(amount);
                }),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;
        let tx_hash = self
            .contract
            .signed_call(
                WrappedTokenFunctions::Wrap.as_str(),
                (),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                    options.value = Some(amount);
                }),
                signer,
            )
            .await?;
        get_blockchain_logger().log(
            format!("Wrapped {:?} on {:?}", amount, self.address()),
            tx_hash,
        );
        Ok(tx_hash)
    }

    pub async fn unwrap(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        amount: U256,
    ) -> Result<H256, RpcCallError> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                WrappedTokenFunctions::Unwrap.as_str(),
                amount,
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;
        let tx_hash = self
            .contract
            .signed_call(
                WrappedTokenFunctions::Unwrap.as_str(),
                amount,
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;
        get_blockchain_logger().log(
            format!("Unwrapped {:?} on {:?}", amount, self.address()),
            tx_hash,
        );
        Ok(tx_hash)
    }

    pub async fn approve(
        &self,
        conn: &EthereumRpcConnection,
        signer: impl Key,
        spender: Address,
        amount: U256,
    ) -> Result<H256, RpcCallError> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                WrappedTokenFunctions::Approve.as_str(),
                (spender, amount),
                signer.address(),
                Options::default(),
            )
            .await?;

        let estimated_gas_price = conn.eth().gas_price().await?;
        let tx_hash = self
            .contract
            .signed_call(
                WrappedTokenFunctions::Approve.as_str(),
                (spender, amount),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.gas_price = Some(estimated_gas_price);
                }),
                signer,
            )
            .await?;
        get_blockchain_logger().log(
            format!(
                "Approved {:?} on {:?}",
                amount_to_display(amount),
                self.address()
            ),
            tx_hash,
        );
        Ok(tx_hash)
    }

    pub async fn balance_of(&self, owner: Address) -> Result<U256, RpcCallError> {
        Ok(self
            .contract
            .query(
                WrappedTokenFunctions::BalanceOf.as_str(),
                owner,
                None,
                Options::default(),
                None,
            )
            .await?)
    }
}

enum WrappedTokenFunctions {
    Wrap,
    Unwrap,
    Approve,
    BalanceOf,
}

impl WrappedTokenFunctions {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Wrap => "deposit",
            Self::Unwrap => "withdraw",
            Self::Approve => "approve",
            Self::BalanceOf => "balanceOf",
        }
    }
}
