use crate::deploy_contract;
use eyre::*;
use web3::contract::Contract;
use web3::signing::Key;
use web3::types::Address;
use web3::{Transport, Web3};

const POOL_ABI_JSON: &str = include_str!("../../../../../../abi/internal/strategy_pool.json");

#[derive(Debug, Clone)]
pub struct StrategyPoolContract<T: Transport> {
    contract: Contract<T>,
    w3: Web3<T>,
}

impl<T: Transport> StrategyPoolContract<T> {
    // only for testing
    pub async fn deploy(w3: Web3<T>, key: impl Key, name: String, symbol: String) -> Result<Self> {
        let params = (name.clone(), symbol.clone(), key.address());
        let contract = deploy_contract(w3.clone(), key, params, "StrategyPool").await?;
        Ok(Self { contract, w3 })
    }
    pub fn new(w3: Web3<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(w3.eth(), address, POOL_ABI_JSON.as_bytes())?;
        Ok(Self { contract, w3 })
    }
    pub fn address(&self) -> Address {
        self.contract.address()
    }
}
