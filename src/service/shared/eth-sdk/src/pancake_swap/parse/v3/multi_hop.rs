use eyre::*;
use web3::types::{H160, U256};

use crate::ContractCall;

#[derive(Debug, Clone)]
pub struct ExactInputParams {
    pub path: Vec<u8>,
    pub recipient: H160,
    pub amount_in: U256,
    pub amount_out_minimum: U256,
}

pub fn exact_input(call: &ContractCall) -> Result<ExactInputParams> {
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

    let params = call.get_param("params")?.get_value().into_tuple()?;

    Ok(ExactInputParams {
        path: params[0].into_bytes()?,
        recipient: params[1].into_address()?,
        amount_in: params[2].into_uint()?,
        amount_out_minimum: params[3].into_uint()?,
    })
}

#[derive(Debug, Clone)]
pub struct ExactOutputParams {
    pub path: Vec<u8>,
    pub recipient: H160,
    pub amount_out: U256,
    pub amount_in_maximum: U256,
}

pub fn exact_output(call: &ContractCall) -> Result<ExactOutputParams> {
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

    let params = call.get_param("params")?.get_value().into_tuple()?;

    Ok(ExactOutputParams {
        path: params[0].into_bytes()?,
        recipient: params[1].into_address()?,
        amount_out: params[2].into_uint()?,
        amount_in_maximum: params[3].into_uint()?,
    })
}
