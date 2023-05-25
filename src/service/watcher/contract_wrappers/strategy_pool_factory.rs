use eyre::*;
use web3::api::Eth;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::Transport;

const FACTORY_ABI_JSON: &str = include_str!("../../../../abi/internal/strategy_pool_factory.json");

#[derive(Debug, Clone)]
pub struct StrategyPoolFactoryContract<T: Transport> {
    inner: Contract<T>,
}

impl<T: Transport> StrategyPoolFactoryContract<T> {
    pub fn new(eth: Eth<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(eth, address, FACTORY_ABI_JSON.as_bytes())?;
        Ok(Self { inner: contract })
    }

    pub async fn create_pool(
        &self,
        signer: impl Key,
        name: String,
        symbol: String,
    ) -> Result<H256> {
        let params = (name, symbol);
        let estimated_gas = self
            .inner
            .estimate_gas(
                StrategyPoolFactoryFunctions::CreatePool.as_str(),
                params.clone(),
                signer.address(),
                Options::default(),
            )
            .await?;

        Ok(self
            .inner
            .signed_call(
                StrategyPoolFactoryFunctions::CreatePool.as_str(),
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?)
    }

    pub async fn get_pool(&self, index: U256) -> Result<Address> {
        Ok(self
            .inner
            .query(
                StrategyPoolFactoryFunctions::GetPool.as_str(),
                index,
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn get_pools(&self) -> Result<Vec<Address>> {
        Ok(self
            .inner
            .query(
                StrategyPoolFactoryFunctions::GetPools.as_str(),
                (),
                None,
                Options::default(),
                None,
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
                StrategyPoolFactoryFunctions::TransferOwnership.as_str(),
                new_owner,
                by,
                Options::default(),
            )
            .await?;

        Ok(self
            .inner
            .signed_call(
                StrategyPoolFactoryFunctions::TransferOwnership.as_str(),
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
                StrategyPoolFactoryFunctions::Owner.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }
}

enum StrategyPoolFactoryFunctions {
    CreatePool,
    GetPools,
    GetPool,
    TransferOwnership,
    Owner,
}

impl StrategyPoolFactoryFunctions {
    fn as_str(&self) -> &'static str {
        match self {
            Self::CreatePool => "createPool",
            Self::GetPools => "getPools",
            Self::GetPool => "getPool",
            Self::TransferOwnership => "transferOwnership",
            Self::Owner => "owner",
        }
    }
}
