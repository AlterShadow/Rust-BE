use crate::evm::DexPath;

use crate::dex_tracker::pancake_swap::pancake::Swap;
use crate::ContractCall;
use eyre::*;
use web3::types::H160;

pub fn swap_exact_tokens_for_tokens(call: &ContractCall) -> Result<Swap> {
    /*
            function swapExactTokensForTokens(
                                                    uint256 amountIn,
                                                    uint256 amountOutMin,
                                                    address[] calldata path,
                                                    address to
                    ) external payable returns (uint256 amountOut);
    */
    let amount_in = call.get_param("amountIn")?.get_value().into_uint()?;
    let amount_out_min = call.get_param("amountOutMin")?.get_value().into_uint()?;
    let path_result: Result<Vec<H160>> = call
        .get_param("path")?
        .get_value()
        .into_array()?
        .iter()
        .map(|token| token.into_address())
        .collect();
    let path = path_result?;
    let token_in = path[0];
    let token_out = path[path.len() - 1];
    let recipient = call.get_param("to")?.get_value().into_address()?;

    Ok(Swap {
        recipient,
        token_in,
        token_out,
        amount_in: Some(amount_in),
        amount_out: None,
        amount_out_minimum: Some(amount_out_min),
        amount_in_maximum: None,
        path: DexPath::PancakeV2(path.to_vec()),
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

    let amount_out = call.get_param("amountOut")?.get_value().into_uint()?;
    let amount_in_max = call.get_param("amountInMax")?.get_value().into_uint()?;
    let path_result: Result<Vec<H160>> = call
        .get_param("path")?
        .get_value()
        .into_array()?
        .iter()
        .map(|token| token.into_address())
        .collect();
    let path = path_result?;
    let token_in = path[0];
    let token_out = path[path.len() - 1];
    let recipient = call.get_param("to")?.get_value().into_address()?;

    Ok(Swap {
        recipient,
        token_in,
        token_out,
        amount_in: None,
        amount_out: Some(amount_out),
        amount_out_minimum: None,
        amount_in_maximum: Some(amount_in_max),
        path: DexPath::PancakeV2(path.to_vec()),
    })
}
