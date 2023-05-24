use eyre::*;
use web3::api::Eth;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::Transport;

const ESCROW_ABI_JSON: &str = include_str!("../../../../abi/internal/escrow.json");

#[derive(Debug, Clone)]
pub struct EscrowContract<T: Transport> {
    inner: Contract<T>,
}

impl<T: Transport> EscrowContract<T> {
    pub fn new(eth: Eth<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(eth, address, ESCROW_ABI_JSON.as_bytes())?;
        Ok(Self { inner: contract })
    }

    pub async fn transfer_token_to(
        &self,
        secret: impl Key,
        by: Address,
        token_address: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<H256> {
        let params = (token_address, recipient, amount);
        let estimated_gas = self
            .inner
            .estimate_gas(
                EscrowFunctions::TransferTokenTo.as_str(),
                params,
                by,
                Options::default(),
            )
            .await?;

        Ok(self
            .inner
            .signed_call(
                EscrowFunctions::TransferTokenTo.as_str(),
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                secret,
            )
            .await?)
    }

    pub async fn transfer_ownership(
        &self,
        secret: impl Key,
        by: Address,
        new_owner: Address,
    ) -> Result<H256> {
        let estimated_gas = self
            .inner
            .estimate_gas(
                EscrowFunctions::TransferOwnership.as_str(),
                new_owner,
                by,
                Options::default(),
            )
            .await?;

        Ok(self
            .inner
            .signed_call(
                EscrowFunctions::TransferOwnership.as_str(),
                new_owner,
                Options::with(|options| options.gas = Some(estimated_gas)),
                secret,
            )
            .await?)
    }

    pub async fn owner(&self) -> Result<Address> {
        Ok(self
            .inner
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
