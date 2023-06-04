use eyre::*;

use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::{Transport, Web3};

const WRAPPED_ABI_JSON: &str = include_str!("weth.json");

#[derive(Debug, Clone)]
pub struct WrappedTokenContract<T: Transport> {
    contract: Contract<T>,
    w3: Web3<T>,
}

impl<T: Transport> WrappedTokenContract<T> {
    pub fn new(w3: Web3<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(w3.eth(), address, WRAPPED_ABI_JSON.as_bytes())?;
        Ok(Self { contract, w3 })
    }

    pub fn address(&self) -> Address {
        self.contract.address()
    }

    pub async fn wrap(&self, signer: impl Key, amount: U256) -> Result<H256> {
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

        Ok(self
            .contract
            .signed_call(
                WrappedTokenFunctions::Wrap.as_str(),
                (),
                Options::with(|options| {
                    options.gas = Some(estimated_gas);
                    options.value = Some(amount);
                }),
                signer,
            )
            .await?)
    }

    pub async fn unwrap(&self, signer: impl Key, amount: U256) -> Result<H256> {
        let estimated_gas = self
            .contract
            .estimate_gas(
                WrappedTokenFunctions::Unwrap.as_str(),
                amount,
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(self
            .contract
            .signed_call(
                WrappedTokenFunctions::Unwrap.as_str(),
                amount,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn approve(&self, signer: impl Key, spender: Address, amount: U256) -> Result<H256> {
        let params = (spender, amount);
        let estimated_gas = self
            .contract
            .estimate_gas(
                WrappedTokenFunctions::Approve.as_str(),
                params,
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(self
            .contract
            .signed_call(
                WrappedTokenFunctions::Approve.as_str(),
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn balance_of(&self, owner: Address) -> Result<U256> {
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
