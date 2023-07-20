use std::str::FromStr;

use eyre::*;
use web3::types::U256;

// TODO: add ethereum_types dependency to use U512 and make scaled arithmetic safe for ALL U256 values

pub trait ScaledMath {
    fn mul_f64(&self, factor: f64) -> Result<U256>;
    fn div_as_f64(&self, divisor: U256) -> Result<f64>;
    fn mul_div(&self, factor: U256, divisor: U256) -> Result<U256>;
    fn mul_div_rounding_up(&self, factor: U256, divisor: U256) -> Result<U256>;
    fn try_checked_add(&self, term: U256) -> Result<U256>;
    fn try_checked_sub(&self, term: U256) -> Result<U256>;
    fn try_checked_mul(&self, factor: U256) -> Result<U256>;
    fn try_checked_div(&self, divisor: U256) -> Result<U256>;
    fn try_checked_div_rounding_up(&self, divisor: U256) -> Result<U256>;
    fn remove_least_significant_digits(&self, digits: usize) -> Result<U256>;
    fn add_least_significant_digits(&self, digits: usize) -> Result<U256>;
}

impl ScaledMath for U256 {
    fn mul_f64(&self, factor: f64) -> Result<U256> {
        ensure!(factor >= 0.0, "factor must be positive: got {}", factor);

        let decimal_digits = 6;
        let value = U256::from((factor * 10.0f64.powf(decimal_digits as _)).round() as u128);

        Ok(self
            .try_checked_mul(value)?
            .try_checked_div(U256::exp10(decimal_digits))?)
    }

    fn div_as_f64(&self, divisor: U256) -> Result<f64> {
        if divisor == U256::zero() {
            bail!("division by zero");
        }

        /* calculate the number of digits in self and divisor */
        let self_digits = self.to_string().len();
        let divisor_digits = divisor.to_string().len();

        /* calculate the scaling factor based on the number of digits */
        /* use a minimum scaling factor of 16 to assure f64 precision */
        let digit_diff = (divisor_digits.saturating_sub(self_digits)).max(16);

        if digit_diff > 77 {
            bail!("scaling the scale factor would cause overflow");
        }

        /* calculate scale factor */
        let scale_factor = U256::exp10(digit_diff);

        /* scale self */
        let scaled_dividend = self
            .checked_mul(scale_factor)
            .context("overflow when scaling dividend")?;

        /* perform division and get the result as a string */
        let quotient = scaled_dividend / divisor;
        let quotient_str = quotient.to_string();

        let int_str: String;
        let frac_str: String;

        /* determine the integer part and the fraction part */
        if quotient_str.len() > digit_diff {
            /* if quotient has more digits than scaling factor, quotient is larger than 1 */
            /* in this case, the dividend is larger than the divisor */
            /* i.e. both integer part and fraction part are present */
            let (int_part, frac_part) = quotient_str.split_at(quotient_str.len() - digit_diff);
            int_str = int_part.to_string();
            frac_str = frac_part[0..frac_part.len().min(16)].to_string();
        } else if quotient_str.len() == digit_diff {
            /* if quotient has same digits as scaling factor, quotient is smaller than 1 */
            /* in this case, the dividend is smaller than the divisor and the quotient has one less digit than the scaled dividend */
            /* i.e. only the fraction part is present */
            int_str = "0".to_string();
            frac_str = quotient_str[0..quotient_str.len().min(16)].to_string();
        } else {
            /* if quotient has less digits than the scaling factor, the quotient is also smaller than 1 */
            /* in this case, the dividend is smaller than the divisor and the quotient has more than one digit less than the scaled dividend */
            /* i.e. need to add leading zeros to the fraction part */
            int_str = "0".to_string();
            let leading_zeros = "0".repeat(digit_diff - quotient_str.len());
            frac_str = (leading_zeros + &quotient_str[0..quotient_str.len().min(16)]).to_string();
        }

        /* construct the final string */
        let result_str = if frac_str.is_empty() {
            int_str.to_string()
        } else {
            format!("{}.{}", int_str, frac_str)
        };

        /* parse string to f64 */
        let result_f64 = f64::from_str(&result_str).context("failed to convert string to f64")?;

        Ok(result_f64)
    }

    fn mul_div(&self, factor: U256, divisor: U256) -> Result<U256> {
        Ok(self.try_checked_mul(factor)?.try_checked_div(divisor)?)
    }

    fn mul_div_rounding_up(&self, factor: U256, divisor: U256) -> Result<U256> {
        Ok(self
            .try_checked_mul(factor)?
            .try_checked_div_rounding_up(divisor)?)
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

    fn try_checked_div_rounding_up(&self, divisor: U256) -> Result<U256> {
        /* add the divisor minus one to the dividend */
        /* standard idiom for integer division rounding up */
        Ok(self
            .try_checked_add(divisor.try_checked_sub(U256::one())?)?
            .try_checked_div(divisor)?)
    }

    fn remove_least_significant_digits(&self, digits: usize) -> Result<U256> {
        let divisor = U256::exp10(digits);
        self.try_checked_div(divisor)
    }

    fn add_least_significant_digits(&self, digits: usize) -> Result<U256> {
        let multiplier = U256::exp10(digits);
        self.try_checked_mul(multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_f64_does_not_change_number_of_digits_from_decimals() {
        let ten = U256::from(10);
        assert_eq!(ten.mul_f64(1.0).unwrap(), U256::from(10));
        assert_eq!(ten.mul_f64(1.5).unwrap(), U256::from(15));
        assert_eq!(ten.mul_f64(1.05).unwrap(), U256::from(10));
        assert_eq!(ten.mul_f64(1.15).unwrap(), U256::from(11));
    }

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

    #[test]
    fn div_as_f64_expected_values() -> Result<()> {
        let mut dividend = U256::from(4);
        let mut divisor = U256::from(2);
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(2));

        dividend = U256::from(1);
        divisor = U256::from(2);
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0.5));

        dividend = U256::from(1);
        divisor = U256::from(3);
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0.3333333333333333));

        dividend = U256::from(10);
        divisor = U256::from(9);
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(1.1111111111111112));

        dividend = U256::from(100000);
        divisor = U256::from(999999);
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0.1000001000001));

        dividend = U256::zero();
        divisor = U256::from(3);
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0));

        dividend = U256::from(1);
        divisor = U256::max_value();
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0));

        dividend = U256::from(1);
        divisor = U256::from(1000000000);
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0.000000001));

        dividend = U256::from(1);
        divisor = U256::from(i64::from_str("4000000000")?);
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0.00000000025));

        dividend = U256::from(1);
        divisor = U256::from(i64::from(10));
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0.1));

        dividend = U256::from(1);
        divisor = U256::from(i64::from(20));
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0.05));

        dividend = U256::from(1);
        divisor = U256::from(i64::from(90));
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0.0111111111111111));

        dividend = U256::from(1);
        divisor = U256::from(i64::from(99));
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0.0101010101010101));

        dividend = U256::from(2);
        divisor = U256::from(200);
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0.01));

        dividend = U256::from(20000);
        divisor = U256::from(i64::from(200));
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(100.0));

        dividend = U256::from(20000);
        divisor = U256::from(i64::from(2000));
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(10.0));

        dividend = U256::from(20000);
        divisor = U256::from(i64::from(1000));
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(20.0));

        dividend = U256::from(20000);
        divisor = U256::from(2);
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(10000.0));

        Ok(())
    }

    #[test]
    fn div_as_f64_with_division_by_zero() {
        let dividend = U256::from(1);
        let divisor = U256::zero();
        match dividend.div_as_f64(divisor) {
            Err(_) => assert!(true), // expected throw an error for division by zero
            _ => assert!(false),
        }
    }

    #[test]
    fn div_as_f64_with_huge_values() -> Result<()> {
        let mut dividend = U256::max_value();
        let mut divisor = U256::exp10(76);
        match dividend.div_as_f64(divisor) {
            Err(_) => assert!(true), // expected throw an error for overflow when scaling
            _ => assert!(false),
        }

        dividend = U256::from(4).try_checked_mul(U256::exp10(17))?;
        divisor = U256::from(2).try_checked_mul(U256::exp10(17))?;
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(2));

        dividend = U256::from(2).try_checked_mul(U256::exp10(17))?;
        divisor = U256::from(4).try_checked_mul(U256::exp10(17))?;
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0.5));

        dividend = U256::from(2).try_checked_mul(U256::exp10(17))?;
        divisor = U256::from(20).try_checked_mul(U256::exp10(17))?;
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(0.1));

        dividend = U256::from(200).try_checked_mul(U256::exp10(17))?;
        divisor = U256::from(150).try_checked_mul(U256::exp10(17))?;
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(1.3333333333333333));

        dividend = U256::from(1000).try_checked_mul(U256::exp10(20))?;
        divisor = U256::from(150).try_checked_mul(U256::exp10(20))?;
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(6.666666666666667));

        dividend = U256::from(4).try_checked_mul(U256::exp10(40))?;
        divisor = U256::from(2).try_checked_mul(U256::exp10(40))?;
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(2));

        dividend = U256::from(4).try_checked_mul(U256::exp10(60))?;
        divisor = U256::from(2).try_checked_mul(U256::exp10(60))?;
        assert_eq!(dividend.div_as_f64(divisor)?, f64::from(2));

        dividend = U256::from(4).try_checked_mul(U256::exp10(61))?;
        divisor = U256::from(2).try_checked_mul(U256::exp10(61))?;
        match dividend.div_as_f64(divisor) {
            Err(_) => assert!(true), // expected throw an error for overflow when scaling
            _ => assert!(false),
        }

        Ok(())
    }
}
