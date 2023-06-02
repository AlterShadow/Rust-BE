use crate::contract::{read_abi_from_solc_output, ContractDeployer};
use eyre::*;
use web3::contract::tokens::Tokenize;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::{Transport, Web3};

pub mod escrow;
pub mod pancake_swap;
pub mod strategy_pool;
pub mod strategy_pool_factory;

pub async fn deploy_contract<T: Transport>(
    w3: Web3<T>,
    key: impl Key,
    params: impl Tokenize,
    contract_name: &str,
) -> Result<Contract<T>> {
    let base = crate::contract::get_project_root()
        .parent()
        .unwrap()
        .to_owned();

    let abi_json = read_abi_from_solc_output(&base.join(format!(
        "app.mc2.fi-solidity/out/{}.sol/{}.json",
        contract_name, contract_name
    )))?;
    let bin = std::fs::read_to_string(base.join(format!(
        "app.mc2.fi-solidity/out/{}.sol/{}.bin",
        contract_name, contract_name
    )))?;
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

    Ok(deployer.sign_with_key_and_execute(params, key).await?)
}
