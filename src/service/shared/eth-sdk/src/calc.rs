use eyre::*;

use web3::types::U256;

pub trait ScaledMath {
    fn mul_f64(&self, factor: f64) -> Result<U256>;
    fn mul_div(&self, factor: U256, divisor: U256) -> Result<U256>;
    fn try_checked_add(&self, term: U256) -> Result<U256>;
    fn try_checked_sub(&self, term: U256) -> Result<U256>;
    fn try_checked_mul(&self, factor: U256) -> Result<U256>;
    fn try_checked_div(&self, divisor: U256) -> Result<U256>;
}

impl ScaledMath for U256 {
    fn mul_f64(&self, factor: f64) -> Result<U256> {
        /* determine the number of relevant decimal places in the f64 */
        let decimals = factor
            .to_string()
            .split('.')
            .nth(1)
            .map(|s| s.len())
            .unwrap_or(0);

        /* calculate a scaling factor based on the number of decimals */
        let multiplier = U256::exp10(decimals);

        /* convert f64 to U256 with proper scaling */
        let f_as_u256: U256 =
            U256::from_dec_str(&format!("{:.0}", factor * 10f64.powi(decimals as i32)))
                .map_err(|_| eyre!("failed to convert f64 to U256"))?;

        /* perform the multiplication with U256 values */
        let result_u256: U256 = self
            .checked_mul(f_as_u256)
            .ok_or_else(|| eyre!("scaled multiplication would overflow"))?;

        Ok(result_u256 / multiplier)
    }

    fn mul_div(&self, factor: U256, divisor: U256) -> Result<U256> {
        /* check if multiplication overflows */
        let mul_result = self
            .checked_mul(factor)
            .ok_or_else(|| eyre!("multiplication would cause overflow"))?;

        /* check if division underflows */
        let div_result = mul_result
            .checked_div(divisor)
            .ok_or_else(|| eyre!("division by zero"))?;

        Ok(div_result)
    }

    fn try_checked_add(&self, term: U256) -> Result<U256> {
        self.checked_add(term)
            .ok_or_else(|| eyre!("addition would cause overflow"))
    }

    fn try_checked_sub(&self, term: U256) -> Result<U256> {
        self.checked_sub(term)
            .ok_or_else(|| eyre!("subtraction would cause underflow"))
    }

    fn try_checked_mul(&self, factor: U256) -> Result<U256> {
        self.checked_mul(factor)
            .ok_or_else(|| eyre!("multiplication would cause overflow"))
    }

    fn try_checked_div(&self, divisor: U256) -> Result<U256> {
        self.checked_div(divisor)
            .ok_or_else(|| eyre!("division by zero"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_f64_with_overflow() {
        let large_value = U256::max_value();
        match large_value.mul_f64(2.0) {
            Err(_) => assert!(true), // expected to overflow
            _ => assert!(false),
        }
    }

    #[test]
    fn mul_div_with_overflow() {
        let large_value = U256::max_value();
        match large_value.mul_div(large_value, U256::from(1)) {
            Err(_) => assert!(true), // expected to overflow
            _ => assert!(false),
        }
    }

    #[test]
    fn mul_div_with_division_by_zero() {
        let x = U256::from(1);
        match x.mul_div(U256::from(1), U256::zero()) {
            Err(_) => assert!(true), // expected throw an error for division by zero
            _ => assert!(false),
        }
    }
}
