use ethabi::Token;
use eyre::*;

use crate::tracker::calldata::ContractCall;
use crate::tracker::pancake_swap::pancake::Swap;
use crate::tracker::trade::{PancakeV3SingleHopPath, Path};

use crate::tracker::ethabi_to_web3::{convert_h160_ethabi_to_web3, convert_u256_ethabi_to_web3};

pub fn exact_input_single(call: &ContractCall) -> Result<Swap> {
    /*
                    function exactInputSingle(
                                    ExactInputSingleParams memory params
                    ) external payable override nonReentrant returns (uint256 amountOut)

                                                    struct ExactInputSingleParams {
                                                                    address tokenIn;
                                                                    address tokenOut;
                                                                    uint24 fee;
                                                                    address recipient;
                                                                    uint256 amountIn;
                                                                    uint256 amountOutMinimum;
                                                                    uint160 sqrtPriceLimitX96;
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

    let token_in = match params[0] {
        Token::Address(param) => convert_h160_ethabi_to_web3(param),
        _ => {
            return Err(eyre!("token_in is not an address"));
        }
    };

    let token_out = match params[1] {
        Token::Address(param) => convert_h160_ethabi_to_web3(param),
        _ => {
            return Err(eyre!("token_out is not an address"));
        }
    };

    let fee = match params[2] {
        Token::Uint(param) => convert_u256_ethabi_to_web3(param),
        _ => {
            return Err(eyre!("fee is not a uint"));
        }
    };

    let recipient = match params[3] {
        Token::Address(param) => convert_h160_ethabi_to_web3(param),
        _ => {
            return Err(eyre!("recipient is not an address"));
        }
    };

    let amount_in = match params[4] {
        Token::Uint(param) => convert_u256_ethabi_to_web3(param),
        _ => {
            return Err(eyre!("amount_in is not a uint"));
        }
    };

    let amount_out_minimum = match params[5] {
        Token::Uint(param) => convert_u256_ethabi_to_web3(param),
        _ => {
            return Err(eyre!("amount_out_minimum is not a uint"));
        }
    };

    Ok(Swap {
        recipient: recipient,
        token_in: token_in,
        token_out: token_out,
        amount_in: Some(amount_in),
        amount_out: None,
        amount_out_minimum: Some(amount_out_minimum),
        amount_in_maximum: None,
        path: Path::PancakeV3SingleHop(PancakeV3SingleHopPath {
            token_in: token_in,
            token_out: token_out,
            fee: fee,
        }),
    })
}

pub fn exact_output_single(call: &ContractCall) -> Result<Swap> {
    /*
                    function exactOutputSingle(
                                    ExactOutputSingleParams calldata params
                    ) external payable override nonReentrant returns (uint256 amountIn)

                                                    struct ExactOutputSingleParams {
                                                                    address tokenIn;
                                                                    address tokenOut;
                                                                    uint24 fee;
                                                                    address recipient;
                                                                    uint256 amountOut;
                                                                    uint256 amountInMaximum;
                                                                    uint160 sqrtPriceLimitX96;
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

    let token_in = match params[0] {
        Token::Address(param) => convert_h160_ethabi_to_web3(param),
        _ => {
            return Err(eyre!("token_in is not an address"));
        }
    };

    let token_out = match params[1] {
        Token::Address(param) => convert_h160_ethabi_to_web3(param),
        _ => {
            return Err(eyre!("token_out is not an address"));
        }
    };

    let fee = match params[2] {
        Token::Uint(param) => convert_u256_ethabi_to_web3(param),
        _ => {
            return Err(eyre!("fee is not a uint"));
        }
    };

    let recipient = match params[3] {
        Token::Address(param) => convert_h160_ethabi_to_web3(param),
        _ => {
            return Err(eyre!("recipient is not an address"));
        }
    };

    let amount_out = match params[4] {
        Token::Uint(param) => convert_u256_ethabi_to_web3(param),
        _ => {
            return Err(eyre!("amount_out is not a uint"));
        }
    };

    let amount_in_maximum = match params[5] {
        Token::Uint(param) => convert_u256_ethabi_to_web3(param),
        _ => {
            return Err(eyre!("amount_in_maximum is not a uint"));
        }
    };

    Ok(Swap {
        recipient: recipient,
        token_in: token_in,
        token_out: token_out,
        amount_in: None,
        amount_out: Some(amount_out),
        amount_out_minimum: None,
        amount_in_maximum: Some(amount_in_maximum),
        path: Path::PancakeV3SingleHop(PancakeV3SingleHopPath {
            token_in: token_in,
            token_out: token_out,
            fee: fee,
        }),
    })
}
