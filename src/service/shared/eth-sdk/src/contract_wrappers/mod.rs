use crate::contract::{read_abi_from_solc_output, ContractDeployer};
use crate::utils::get_project_root;
use eyre::*;
use tracing::info;
use web3::contract::tokens::Tokenize;
use web3::contract::{Contract, Options};
use web3::signing::Key;
use web3::{Transport, Web3};

pub mod erc20;
pub mod escrow;
pub mod mock_erc20;
pub mod strategy_pool;
pub mod wrapped_token;

pub async fn deploy_contract<T: Transport>(
    w3: Web3<T>,
    key: impl Key,
    params: impl Tokenize,
    contract_name: &str,
) -> Result<Contract<T>> {
    let base = get_project_root().parent().unwrap().to_owned();
    let abi_json_path = &base.join(format!(
        "app.mc2.fi-solidity/out/{}.sol/{}.json",
        contract_name, contract_name
    ));
    info!("Reading {}", abi_json_path.display());
    let abi_json = read_abi_from_solc_output(abi_json_path)?;
    let bin_path = base.join(format!(
        "app.mc2.fi-solidity/out/{}.sol/{}.bin",
        contract_name, contract_name
    ));
    info!("Reading {}", bin_path.display());
    let bin = std::fs::read_to_string(bin_path)?;
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
