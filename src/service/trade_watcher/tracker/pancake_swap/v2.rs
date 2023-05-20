use ethabi::Token;
use eyre::*;
use web3::types::H160;

use lib::evm_parse::calldata::ContractCall;
use lib::evm_parse::ethabi_to_web3::{convert_h160_ethabi_to_web3, convert_u256_ethabi_to_web3};

use crate::tracker::pancake_swap::pancake::Swap;
use crate::tracker::trade::Path;

pub fn swap_exact_tokens_for_tokens(call: &ContractCall) -> Result<Swap> {
    /*
            function swapExactTokensForTokens(
                                                    uint256 amountIn,
                                                    uint256 amountOutMin,
                                                    address[] calldata path,
                                                    address to
                    ) external payable returns (uint256 amountOut);
    */
    let amount_in = match call.get_param("amountIn") {
        Some(param) => match param.get_value() {
            Token::Uint(value) => convert_u256_ethabi_to_web3(value),
            _ => {
                return Err(eyre!("amountIn is not a uint"));
            }
        },
        None => {
            return Err(eyre!("no amountIn"));
        }
    };

    let amount_out_min = match call.get_param("amountOutMin") {
        Some(param) => match param.get_value() {
            Token::Uint(value) => convert_u256_ethabi_to_web3(value),
            _ => {
                return Err(eyre!("amountOutMin is not a uint"));
            }
        },
        None => {
            return Err(eyre!("no amountOutMin"));
        }
    };

    let path: Vec<H160> = match call.get_param("path") {
        Some(param) => match param.get_value() {
            Token::Array(value) => value
                .iter()
                .map(|token| match token {
                    Token::Address(value) => Ok(convert_h160_ethabi_to_web3(*value)),
                    _ => Err(eyre!("token in path is not an address")),
                })
                .collect::<Result<Vec<_>, _>>()?,
            _ => {
                return Err(eyre!("path is not an array"));
            }
        },
        None => {
            return Err(eyre!("no path"));
        }
    };

    let token_in = path[0].clone();

    let token_out = path[path.len() - 1].clone();

    let recipient = match call.get_param("to") {
        Some(param) => match param.get_value() {
            Token::Address(value) => convert_h160_ethabi_to_web3(value),
            _ => {
                return Err(eyre!("recipient is not an address"));
            }
        },
        None => {
            return Err(eyre!("no recipient"));
        }
    };

    Ok(Swap {
        recipient: recipient,
        token_in: token_in,
        token_out: token_out,
        amount_in: Some(amount_in),
        amount_out: None,
        amount_out_minimum: Some(amount_out_min),
        amount_in_maximum: None,
        path: Path::PancakeV2(path.to_vec()),
    })
}

pub fn swap_tokens_for_exact_tokens(call: &ContractCall) -> Result<Swap> {
    /*
            function swapTokensForExactTokens(
                                    uint256 amountOut,
                                    uint256 amountInMax,
                                    address[] calldata path,
                                    address to
            ) external payable override nonReentrant returns (uint256 amountIn)
    */

    let amount_out = match call.get_param("amountOut") {
        Some(param) => match param.get_value() {
            Token::Uint(value) => convert_u256_ethabi_to_web3(value),
            _ => {
                return Err(eyre!("amountOut is not a uint"));
            }
        },
        None => {
            return Err(eyre!("no amountOut"));
        }
    };

    let amount_in_max = match call.get_param("amountInMax") {
        Some(param) => match param.get_value() {
            Token::Uint(value) => convert_u256_ethabi_to_web3(value),
            _ => {
                return Err(eyre!("amountInMax is not a uint"));
            }
        },
        None => {
            return Err(eyre!("no amountInMax"));
        }
    };

    let path: Vec<H160> = match call.get_param("path") {
        Some(param) => match param.get_value() {
            Token::Array(value) => value
                .iter()
                .map(|token| match token {
                    Token::Address(value) => Ok(convert_h160_ethabi_to_web3(*value)),
                    _ => Err(eyre!("token in path is not an address")),
                })
                .collect::<Result<Vec<_>, _>>()?,
            _ => {
                return Err(eyre!("path is not an array"));
            }
        },
        None => {
            return Err(eyre!("no path"));
        }
    };

    let token_in = path[0].clone();

    let token_out = path[1].clone();

    let recipient = match call.get_param("to") {
        Some(param) => match param.get_value() {
            Token::Address(value) => convert_h160_ethabi_to_web3(value),
            _ => {
                return Err(eyre!("recipient is not an address"));
            }
        },
        None => {
            return Err(eyre!("no recipient"));
        }
    };

    Ok(Swap {
        recipient: recipient,
        token_in: token_in,
        token_out: token_out,
        amount_in: None,
        amount_out: Some(amount_out),
        amount_out_minimum: None,
        amount_in_maximum: Some(amount_in_max),
        path: Path::PancakeV2(path.to_vec()),
    })
}
