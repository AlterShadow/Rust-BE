use eyre::*;
use num::ToPrimitive;
use web3::types::{Address, H256, U256};

use eth_sdk::ScaledMath;

pub fn normalize_decimals_to(
    normalize_to: usize,
    token_amount: U256,
    token_decimals: usize,
) -> Result<U256> {
    if normalize_to > token_decimals {
        Ok(token_amount.try_checked_mul(U256::exp10(normalize_to - token_decimals))?)
    } else {
        Ok(token_amount.try_checked_div(U256::exp10(token_decimals - normalize_to))?)
    }
}
