use crate::contract::{read_abi_from_solc_output, ContractDeployer};
use crate::utils::get_project_root;
use eyre::*;
use lib::log::DynLogger;
use tracing::info;
use web3::contract::tokens::Tokenize;
use web3::contract::{Contract, Options};
use web3::ethabi;
use web3::signing::Key;
use web3::types::Bytes;
use web3::{Transport, Web3};

pub mod erc20;
pub mod escrow;
pub mod mock_erc20;
pub mod new_escrow;
pub mod strategy_pool;
pub mod strategy_pool_herald;
pub mod strategy_wallet;
pub mod wrapped_token;

pub async fn deploy_contract<T: Transport>(
    w3: Web3<T>,
    key: impl Key + Clone,
    params: impl Tokenize + Clone,
    contract_name: &str,
    logger: DynLogger,
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

    /* encode constructor parameters */
    let constructor_params = ethabi::encode(&params.clone().into_tokens());

    /* decode contract bytecode from hex */
    let bin_bytes = hex::decode(&bin)?;

    /* append encoded parameters to bytecode bytes */
    let code_with_constructor = [bin_bytes, constructor_params].concat();
    logger.log(format!("estimating gas for deploying {}", contract_name));
    /* estimate gas */
    let estimated_gas = w3
        .eth()
        .estimate_gas(
            web3::types::CallRequest {
                from: None,
                to: None, // for contract deployment this must be None
                gas: None,
                gas_price: None,
                value: None,
                data: Some(Bytes::from(code_with_constructor)),
                transaction_type: None,
                access_list: None,
                max_fee_per_gas: None,
                max_priority_fee_per_gas: None,
            },
            None,
        )
        .await?;

    /* estimate gas price */
    let estimated_gas_price = w3.eth().gas_price().await?;
    logger.log(format!(
        "estimated gas: {}, estimated gas price: {}",
        estimated_gas, estimated_gas_price
    ));
    /* setup options with GAS estimations */
    let options = Options {
        gas: Some(estimated_gas),
        gas_price: Some(estimated_gas_price),
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

    Ok(deployer
        .sign_with_key_and_execute(params, key, logger)
        .await?)
}
