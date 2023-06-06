use crate::contract::AbstractContract;


use crate::{deploy_contract, EitherTransport, EthereumRpcConnectionPool, MultiChainAddressTable};
use eyre::*;
use gen::model::EnumBlockChain;

use web3::contract::Contract;
use web3::signing::Key;
use web3::types::Address;
use web3::{ethabi, Transport, Web3};

const POOL_ABI_JSON: &str = include_str!("strategy_pool.json");
pub struct AbstractStrategyPoolContract(AbstractContract<()>);
impl AbstractStrategyPoolContract {
    pub fn new(name: String, table: MultiChainAddressTable<()>) -> Self {
        let abi = ethabi::Contract::load(POOL_ABI_JSON.as_bytes()).unwrap();
        Self(AbstractContract {
            name,
            abi,
            contract_addresses: table,
        })
    }

    pub async fn get(
        &self,
        pool: &EthereumRpcConnectionPool,
        blockchain: EnumBlockChain,
    ) -> Result<StrategyPoolContract<EitherTransport>> {
        let contract = self.0.get(pool, blockchain, ()).await?;
        Ok(StrategyPoolContract { contract })
    }
}
#[derive(Debug, Clone)]
pub struct StrategyPoolContract<T: Transport> {
    contract: Contract<T>,
}

impl<T: Transport> StrategyPoolContract<T> {
    // only for testing
    pub async fn deploy(w3: Web3<T>, key: impl Key, name: String, symbol: String) -> Result<Self> {
        let params = (name.clone(), symbol.clone(), key.address());
        let contract = deploy_contract(w3.clone(), key, params, "StrategyPool").await?;
        Ok(Self { contract })
    }
    pub fn new(w3: Web3<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(w3.eth(), address, POOL_ABI_JSON.as_bytes())?;
        Ok(Self { contract })
    }
    pub fn address(&self) -> Address {
        self.contract.address()
    }
}
