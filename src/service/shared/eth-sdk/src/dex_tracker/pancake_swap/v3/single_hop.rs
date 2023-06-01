use crate::dex_tracker::pancake::Swap;
use crate::evm::{DexPath, PancakeV3SingleHopPath};
use crate::ContractCall;
use eyre::*;

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

    let params = call.get_param("params")?.get_value().into_tuple()?;
    let token_in = &params[0].into_address()?;
    let token_out = &params[1].into_address()?;
    let fee = &params[2].into_uint()?;
    let recipient = &params[3].into_address()?;
    let amount_in = &params[4].into_uint()?;
    let amount_out_minimum = &params[5].into_uint()?;

    Ok(Swap {
        recipient: *recipient,
        token_in: *token_in,
        token_out: *token_out,
        amount_in: Some(*amount_in),
        amount_out: None,
        amount_out_minimum: Some(*amount_out_minimum),
        amount_in_maximum: None,
        path: DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
            token_in: *token_in,
            token_out: *token_out,
            fee: *fee,
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

    let params = call.get_param("params")?.get_value().into_tuple()?;
    let token_in = &params[0].into_address()?;
    let token_out = &params[1].into_address()?;
    let fee = &params[2].into_uint()?;
    let recipient = &params[3].into_address()?;
    let amount_out = &params[4].into_uint()?;
    let amount_in_maximum = &params[5].into_uint()?;

    Ok(Swap {
        recipient: *recipient,
        token_in: *token_in,
        token_out: *token_out,
        amount_in: None,
        amount_out: Some(*amount_out),
        amount_out_minimum: None,
        amount_in_maximum: Some(*amount_in_maximum),
        path: DexPath::PancakeV3SingleHop(PancakeV3SingleHopPath {
            token_in: *token_in,
            token_out: *token_out,
            fee: *fee,
        }),
    })
}
