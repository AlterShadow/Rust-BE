use crate::contract_wrappers::strategy_pool_factory::StrategyPoolFactoryContract;
use eth_sdk::contract::{read_abi_from_solc_output, ContractDeployer};
use eyre::*;
use std::time::Duration;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::{Transport, Web3};

const POOL_ABI_JSON: &str = include_str!("../../../../abi/internal/strategy_pool.json");

#[derive(Debug, Clone)]
pub struct StrategyPoolContract<T: Transport> {
    contract: Contract<T>,
    w3: Web3<T>,
}

impl<T: Transport> StrategyPoolContract<T> {
    #[cfg(test)]
    pub async fn deploy(w3: Web3<T>, key: impl Key, name: String, symbol: String) -> Result<Self> {
        let base = eth_sdk::contract::get_project_root()
            .parent()
            .unwrap()
            .to_owned();

        let abi_json = read_abi_from_solc_output(
            &base.join("app.mc2.fi-solidity/out/StrategyPool.sol/StrategyPool.json"),
        )?;
        let bin = std::fs::read_to_string(
            base.join("app.mc2.fi-solidity/out/StrategyPool.sol/StrategyPool.bin"),
        )?;
        // web3::contract::web3 never worked: Abi error: Invalid data for ABI json
        let options = Options {
            gas: Some(20000000.into()),
            gas_price: None,
            value: None,
            nonce: None,
            condition: None,
            transaction_type: None,
            access_list: None,
            max_fee_per_gas: None,
            max_priority_fee_per_gas: None,
        };

        let deployer = ContractDeployer::new(w3.eth(), abi_json)?
            .code(bin)
            .options(options);

        Ok(Self {
            contract: deployer
                .sign_with_key_and_execute((name, symbol, key.address()), key)
                .await?,
            w3,
        })
    }
    pub fn new(w3: Web3<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(w3.eth(), address, POOL_ABI_JSON.as_bytes())?;
        Ok(Self { contract, w3 })
    }
    pub fn address(&self) -> Address {
        self.contract.address()
    }
}
