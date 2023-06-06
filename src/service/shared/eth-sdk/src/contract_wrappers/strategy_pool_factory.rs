use crate::contract::AbstractContract;
use crate::{deploy_contract, EitherTransport, EthereumRpcConnectionPool, MultiChainAddressTable};
use eyre::*;
use gen::model::EnumBlockChain;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::{ethabi, Transport, Web3};

const FACTORY_ABI_JSON: &str = include_str!("strategy_pool_factory.json");

pub struct AbstractStrategyPoolFactoryContract(AbstractContract<()>);
impl AbstractStrategyPoolFactoryContract {
    pub fn new(table: MultiChainAddressTable<()>) -> Self {
        let abi = ethabi::Contract::load(FACTORY_ABI_JSON.as_bytes()).unwrap();
        Self(AbstractContract {
            name: "StrategyPoolFactory".to_string(),
            abi,
            contract_addresses: table,
        })
    }

    pub async fn get(
        &self,
        pool: &EthereumRpcConnectionPool,
        blockchain: EnumBlockChain,
    ) -> Result<StrategyPoolFactoryContract<EitherTransport>> {
        let contract = self.0.get(pool, blockchain, ()).await?;
        Ok(StrategyPoolFactoryContract { contract })
    }
}
#[derive(Debug, Clone)]
pub struct StrategyPoolFactoryContract<T: Transport> {
    contract: Contract<T>,
}

impl<T: Transport> StrategyPoolFactoryContract<T> {
    // #[cfg(test)]
    pub async fn deploy(w3: Web3<T>, key: impl Key) -> Result<Self> {
        let params = (key.address(),);
        let contract = deploy_contract(w3.clone(), key, params, "StrategyPoolFactory").await?;
        Ok(Self { contract })
    }
    pub fn new(w3: Web3<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(w3.eth(), address, FACTORY_ABI_JSON.as_bytes())?;
        Ok(Self { contract })
    }
    pub fn address(&self) -> Address {
        self.contract.address()
    }

    pub async fn get_pool(&self, index: U256) -> Result<Address> {
        Ok(self
            .contract
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
            .contract
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
            .contract
            .estimate_gas(
                StrategyPoolFactoryFunctions::TransferOwnership.as_str(),
                new_owner,
                by,
                Options::default(),
            )
            .await?;

        Ok(self
            .contract
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
            .contract
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
    GetPools,
    GetPool,
    TransferOwnership,
    Owner,
}

impl StrategyPoolFactoryFunctions {
    fn as_str(&self) -> &'static str {
        match self {
            Self::GetPools => "getPools",
            Self::GetPool => "getPool",
            Self::TransferOwnership => "transferOwnership",
            Self::Owner => "owner",
        }
    }
}
