use crate::contract::{get_project_root, read_abi_from_solc_output, ContractDeployer};
use crate::utils::wait_for_confirmations_simple;
use eyre::*;
use std::time::Duration;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::types::{Address, H256, U256};
use web3::{Transport, Web3};

const FACTORY_ABI_JSON: &str =
    include_str!("../../../../../../abi/internal/strategy_pool_factory.json");

#[derive(Debug, Clone)]
pub struct StrategyPoolFactoryContract<T: Transport> {
    contract: Contract<T>,
    w3: Web3<T>,
}

impl<T: Transport> StrategyPoolFactoryContract<T> {
    // #[cfg(test)]
    pub async fn deploy(w3: Web3<T>, key: impl Key) -> Result<Self> {
        let base = get_project_root().parent().unwrap().to_owned();

        let abi_json = read_abi_from_solc_output(
            &base.join("app.mc2.fi-solidity/out/StrategyPoolFactory.sol/StrategyPoolFactory.json"),
        )?;
        let bin = std::fs::read_to_string(
            base.join("app.mc2.fi-solidity/out/StrategyPoolFactory.sol/StrategyPoolFactory.bin"),
        )?;
        // web3::contract::web3 never worked: Abi error: Invalid data for ABI json
        let deployer = ContractDeployer::new(w3.eth(), abi_json)?.code(bin);

        Ok(Self {
            contract: deployer
                .sign_with_key_and_execute(key.address(), key)
                .await?,
            w3,
        })
    }
    pub fn new(w3: Web3<T>, address: Address) -> Result<Self> {
        let contract = Contract::from_json(w3.eth(), address, FACTORY_ABI_JSON.as_bytes())?;
        Ok(Self { contract, w3 })
    }
    pub fn address(&self) -> Address {
        self.contract.address()
    }

    pub async fn create_pool(
        &self,
        signer: impl Key,
        name: String,
        symbol: String,
    ) -> Result<Address> {
        let index = U256::from(self.get_pools().await?.len());
        let params = (index, name, symbol);
        let estimated_gas = self
            .contract
            .estimate_gas(
                StrategyPoolFactoryFunctions::CreatePool.as_str(),
                params.clone(),
                signer.address(),
                Options::default(),
            )
            .await?;
        let tx = self
            .contract
            .signed_call(
                StrategyPoolFactoryFunctions::CreatePool.as_str(),
                params,
                Options::with(|options| options.gas = Some(estimated_gas)),
                signer,
            )
            .await?;
        wait_for_confirmations_simple(&self.w3.eth(), tx, Duration::from_secs(3), 5).await?;
        self.get_pool(index.into()).await
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
