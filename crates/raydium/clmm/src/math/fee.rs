use crate::{constants::FEE_RATE_DENOMINATOR_VALUE, math::full_math::MulDiv};

pub fn calculate_amount_without_fee(amount: u64, fee_rate: u32, round_up: bool) -> Option<u64> {
    if round_up {
        amount.mul_div_ceil(
            (FEE_RATE_DENOMINATOR_VALUE - fee_rate).into(),
            u64::from(FEE_RATE_DENOMINATOR_VALUE),
        )
    } else {
        amount.mul_div_floor(
            (FEE_RATE_DENOMINATOR_VALUE - fee_rate).into(),
            u64::from(FEE_RATE_DENOMINATOR_VALUE),
        )
    }
}

pub fn calculate_fee(amount: u64, fee_rate: u32, round_up: bool) -> Option<u64> {
    if round_up {
        amount.mul_div_ceil((fee_rate).into(), u64::from(FEE_RATE_DENOMINATOR_VALUE - fee_rate))
    } else {
        amount.mul_div_floor((fee_rate).into(), u64::from(FEE_RATE_DENOMINATOR_VALUE - fee_rate))
    }
}
