use std::fs::read;

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
        secret: impl Key,
        by: Address,
        trader: Address,
        name: String,
        symbol: String,
        initial_deposit_value: U256,
    ) -> Result<H256> {
        let params = (trader, name, symbol, initial_deposit_value);
        let estimated_gas = self
            .inner
            .estimate_gas(
                StrategyPoolFactoryFunctions::CreatePool.as_str(),
                params.clone(),
                by,
                Options::default(),
            )
            .await?;

        Ok(self
            .inner
            .signed_call(
                StrategyPoolFactoryFunctions::CreatePool.as_str(),
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                secret,
            )
            .await?)
    }

    pub async fn get_pool(&self, trader: Address) -> Result<Address> {
        Ok(self
            .inner
            .query(
                StrategyPoolFactoryFunctions::GetPool.as_str(),
                trader,
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn pool_owner(&self) -> Result<Address> {
        Ok(self
            .inner
            .query(
                StrategyPoolFactoryFunctions::PoolOwner.as_str(),
                (),
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn trader_to_pool(&self, trader: Address) -> Result<Address> {
        Ok(self
            .inner
            .query(
                StrategyPoolFactoryFunctions::TraderToPool.as_str(),
                trader,
                None,
                Options::default(),
                None,
            )
            .await?)
    }

    pub async fn traders(&self) -> Result<Vec<Address>> {
        Ok(self
            .inner
            .query(
                StrategyPoolFactoryFunctions::Traders.as_str(),
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
        let params = (new_owner);
        let estimated_gas = self
            .inner
            .estimate_gas(
                StrategyPoolFactoryFunctions::TransferOwnership.as_str(),
                params,
                by,
                Options::default(),
            )
            .await?;

        Ok(self
            .inner
            .signed_call(
                StrategyPoolFactoryFunctions::TransferOwnership.as_str(),
                params,
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
    GetPool,
    PoolOwner,
    TraderToPool,
    Traders,
    TransferOwnership,
    Owner,
}

impl StrategyPoolFactoryFunctions {
    fn as_str(&self) -> &'static str {
        match self {
            Self::CreatePool => "createPool",
            Self::GetPool => "getPool",
            Self::PoolOwner => "poolOwner",
            Self::TraderToPool => "traderToPool",
            Self::Traders => "traders",
            Self::TransferOwnership => "transferOwnership",
            Self::Owner => "owner",
        }
    }
}
