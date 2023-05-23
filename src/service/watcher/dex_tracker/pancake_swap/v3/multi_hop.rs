use crate::evm::DexPath;

use crate::dex_tracker::pancake::Swap;
use eth_sdk::utils::{convert_h160_ethabi_to_web3, convert_u256_ethabi_to_web3};
use eth_sdk::ContractCall;
use ethabi::Token;
use eyre::*;
use web3::types::{H160, U256};

#[derive(Debug)]
pub struct MultiHopPath {
    first_token: H160,
    fee: U256,
    second_token: H160,
}

impl MultiHopPath {
    fn from_bytes(path: &[u8]) -> Result<Vec<Self>> {
        if path.len() < 43 {
            /* 20 bytes for address, 3 bytes for uint24, 20 bytes for address */
            return Err(eyre!("path is too short"));
        }

        let mut full_path: Vec<MultiHopPath> = Vec::new();
        let mut first_token: H160 = H160::from_slice(&path[0..20]);
        for i in 0..((path.len() - 20) / 23) {
            let start = 20 + i * 23;
            let fee_bytes: [u8; 3] = match path[start..start + 3].try_into() {
                Ok(bytes) => bytes,
                Err(e) => {
                    return Err(eyre!(
                        "Error parsing 'path' from PancakeSwap exactInput call: {}",
                        e
                    ));
                }
            };
            let fee = U256::from(u32::from_be_bytes([
                0,
                fee_bytes[0],
                fee_bytes[1],
                fee_bytes[2],
            ]));
            let second_token: H160 = H160::from_slice(&path[start + 3..start + 23]);
            full_path.push(MultiHopPath {
                first_token,
                fee,
                second_token,
            });
            first_token = second_token;
        }
        Ok(full_path)
    }
    pub fn get_fee(&self) -> U256 {
        self.fee
    }
}

pub fn exact_input(call: &ContractCall) -> Result<Swap> {
    /*
                    function exactInput(
                                    ExactInputParams memory params
                    ) external payable nonReentrant override returns (uint256 amountOut)

                                                    struct ExactInputParams {
                                                                    bytes path;
                                                                    address recipient;
                                                                    uint256 amountIn;
                                                                    uint256 amountOutMinimum;
                                                    }
    */

    let params = match call.get_param("params") {
        Some(param) => match param.get_value() {
            Token::Tuple(value) => value,
            _ => {
                return Err(eyre!("params is not a tuple"));
            }
        },
        None => {
            return Err(eyre!("no params"));
        }
    };

    let path = match &params[0] {
        Token::Bytes(bytes) => bytes,
        _ => {
            return Err(eyre!("path is not bytes"));
        }
    };

    let full_path = MultiHopPath::from_bytes(path)?;

    let recipient = match &params[1] {
        Token::Address(param) => convert_h160_ethabi_to_web3(*param),
        _ => {
            return Err(eyre!("recipient is not an address"));
        }
    };

    let amount_in = match &params[2] {
        Token::Uint(param) => convert_u256_ethabi_to_web3(*param),
        _ => {
            return Err(eyre!("amount_in is not a uint"));
        }
    };

    let amount_out_minimum = match &params[3] {
        Token::Uint(param) => convert_u256_ethabi_to_web3(*param),
        _ => {
            return Err(eyre!("amount_out_minimum is not a uint"));
        }
    };

    Ok(Swap {
        recipient,
        token_in: full_path[0].first_token,
        token_out: full_path[full_path.len() - 1].second_token,
        amount_in: Some(amount_in),
        amount_out: None,
        amount_out_minimum: Some(amount_out_minimum),
        amount_in_maximum: None,
        path: DexPath::PancakeV3MultiHop(path.to_vec()),
    })
}

pub fn exact_output(call: &ContractCall) -> Result<Swap> {
    /*
                    function exactOutput(
                                    ExactOutputParams calldata params
                    ) external payable override nonReentrant returns (uint256 amountIn)

                                                    struct ExactOutputParams {
                                                                    bytes path;
                                                                    address recipient;
                                                                    uint256 amountOut;
                                                                    uint256 amountInMaximum;
                                                    }
    */

    let params = match call.get_param("params") {
        Some(param) => match param.get_value() {
            Token::Tuple(value) => value,
            _ => {
                return Err(eyre!("params is not a tuple"));
            }
        },
        None => {
            return Err(eyre!("no params"));
        }
    };

    let path = match &params[0] {
        Token::Bytes(bytes) => bytes,
        _ => {
            return Err(eyre!("path is not bytes"));
        }
    };

    let full_path = MultiHopPath::from_bytes(path)?;

    let recipient = match &params[1] {
        Token::Address(param) => convert_h160_ethabi_to_web3(*param),
        _ => {
            return Err(eyre!("recipient is not an address"));
        }
    };

    let amount_out = match &params[2] {
        Token::Uint(param) => convert_u256_ethabi_to_web3(*param),
        _ => {
            return Err(eyre!("amount_out is not a uint"));
        }
    };

    let amount_in_maximum = match &params[3] {
        Token::Uint(param) => convert_u256_ethabi_to_web3(*param),
        _ => {
            return Err(eyre!("amount_in_maximum is not a uint"));
        }
    };

    Ok(Swap {
        recipient,
        token_in: full_path[full_path.len() - 1].second_token,
        token_out: full_path[0].first_token,
        amount_in: None,
        amount_out: Some(amount_out),
        amount_out_minimum: None,
        amount_in_maximum: Some(amount_in_maximum),
        path: DexPath::PancakeV3MultiHop(path.to_vec()),
    })
}
