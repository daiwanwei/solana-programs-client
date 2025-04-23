use snafu::{OptionExt, ResultExt, Snafu};

use crate::math::{
    fee::{calculate_amount_without_fee, calculate_fee},
    liquidity, sqrt_price,
};

/// Computes the result of swapping some amount in, or amount out, given the
/// parameters of the swap
pub fn compute_swap_step(
    sqrt_price_current_x64: u128,
    sqrt_price_target_x64: u128,
    liquidity: u128,
    amount_remaining: u64,
    fee_rate: u32,
    is_base_input: bool,
    zero_for_one: bool,
) -> Result<SwapStep> {
    // let exact_in = amount_remaining >= 0;
    if is_base_input {
        compute_swap_step_by_specified_amount_in(
            sqrt_price_current_x64,
            sqrt_price_target_x64,
            liquidity,
            amount_remaining,
            fee_rate,
            zero_for_one,
        )
    } else {
        compute_swap_step_by_specified_amount_out(
            sqrt_price_current_x64,
            sqrt_price_target_x64,
            liquidity,
            amount_remaining,
            fee_rate,
            zero_for_one,
        )
    }
}

pub fn compute_swap_step_by_specified_amount_in(
    sqrt_price_current_x64: u128,
    sqrt_price_target_x64: u128,
    liquidity: u128,
    amount_remaining: u64,
    fee_rate: u32,
    zero_for_one: bool,
) -> Result<SwapStep> {
    // round up amount_in
    // In exact input case, amount_remaining is positive
    // TODO: handle overflow
    let amount_remaining_without_fee =
        calculate_amount_without_fee(amount_remaining, fee_rate, false)
            .context(CalculateFeeOverflowSnafu)?;

    if amount_remaining_without_fee == 0 {
        println!("amount_remaining_without_fee == 0");
        let fee =
            calculate_fee(amount_remaining, fee_rate, true).context(CalculateFeeOverflowSnafu)?;
        return Ok(SwapStep {
            sqrt_price_next_x64: sqrt_price_target_x64,
            amount_in: amount_remaining,
            amount_out: 0,
            fee_amount: fee,
        });
    }

    // TODO: handle overflow
    let amount_in_to_target = calculate_amount_in_range(
        sqrt_price_current_x64,
        sqrt_price_target_x64,
        liquidity,
        zero_for_one,
        true,
    )?
    .context(CalculateAmountInRangeOverflowSnafu)?;

    let (sqrt_price_next_x64, amount_in, fee_amount) =
        if amount_remaining_without_fee >= amount_in_to_target {
            let fee = calculate_fee(amount_in_to_target, fee_rate, true)
                .context(CalculateFeeOverflowSnafu)?;
            (sqrt_price_target_x64, amount_in_to_target + fee, fee)
        } else {
            (
                sqrt_price::get_next_sqrt_price_from_input(
                    sqrt_price_current_x64,
                    liquidity,
                    amount_remaining_without_fee,
                    zero_for_one,
                ),
                amount_remaining,
                u64::from(amount_remaining)
                    .checked_sub(amount_remaining_without_fee)
                    .context(MathOverflowSnafu)?,
            )
        };

    let amount_out = calculate_amount_in_range(
        sqrt_price_current_x64,
        sqrt_price_next_x64,
        liquidity,
        zero_for_one,
        false,
    )?
    .context(CalculateAmountInRangeOverflowSnafu)?;

    let swap_step = SwapStep { amount_in, amount_out, sqrt_price_next_x64, fee_amount };

    Ok(swap_step)
}

pub fn compute_swap_step_by_specified_amount_out(
    sqrt_price_current_x64: u128,
    sqrt_price_target_x64: u128,
    liquidity: u128,
    amount_remaining: u64,
    fee_rate: u32,
    zero_for_one: bool,
) -> Result<SwapStep> {
    // TODO: handle overflow
    let amount_out_to_target = calculate_amount_in_range(
        sqrt_price_current_x64,
        sqrt_price_target_x64,
        liquidity,
        zero_for_one,
        false,
    )?
    .context(CalculateAmountInRangeOverflowSnafu)?;

    // For exact output case, cap the output amount to not exceed the remaining
    // output amount
    let (amount_out, sqrt_price_next_x64) = if amount_out_to_target > amount_remaining {
        (
            amount_remaining,
            sqrt_price::get_next_sqrt_price_from_output(
                sqrt_price_current_x64,
                liquidity,
                amount_remaining,
                zero_for_one,
            ),
        )
    } else {
        (amount_out_to_target, sqrt_price_target_x64)
    };

    let amount_in = calculate_amount_in_range(
        sqrt_price_current_x64,
        sqrt_price_next_x64,
        liquidity,
        zero_for_one,
        true,
    )?
    .context(CalculateAmountInRangeOverflowSnafu)?;

    let fee_amount = calculate_fee(amount_in, fee_rate, true).context(CalculateFeeOverflowSnafu)?;

    let swap_step = SwapStep { amount_in, amount_out, sqrt_price_next_x64, fee_amount };

    Ok(swap_step)
}

/// Pre calcumate amount_in or amount_out for the specified price range
/// The amount maybe overflow of u64 due to the `sqrt_price_target_x64` maybe
/// unreasonable. Therefore, this situation needs to be handled in
/// `compute_swap_step` to recalculate the price that can be reached based on
/// the amount.

fn calculate_amount_in_range(
    sqrt_price_current_x64: u128,
    sqrt_price_target_x64: u128,
    liquidity: u128,
    zero_for_one: bool,
    is_base_input: bool,
) -> Result<Option<u64>> {
    if is_base_input {
        let result = if zero_for_one {
            liquidity::get_delta_amount_0_unsigned(
                sqrt_price_target_x64,
                sqrt_price_current_x64,
                liquidity,
                true,
            )
        } else {
            liquidity::get_delta_amount_1_unsigned(
                sqrt_price_current_x64,
                sqrt_price_target_x64,
                liquidity,
                true,
            )
        }
        .context(LiquiditySnafu)?;
        Ok(Some(result))
    } else {
        let result = if zero_for_one {
            liquidity::get_delta_amount_1_unsigned(
                sqrt_price_target_x64,
                sqrt_price_current_x64,
                liquidity,
                false,
            )
        } else {
            liquidity::get_delta_amount_0_unsigned(
                sqrt_price_current_x64,
                sqrt_price_target_x64,
                liquidity,
                false,
            )
        }
        .context(LiquiditySnafu)?;
        Ok(Some(result))
    }
}

/// Result of a swap step
#[derive(Default, Debug)]
pub struct SwapStep {
    /// The price after swapping the amount in/out, not to exceed the price
    /// target
    pub sqrt_price_next_x64: u128,
    pub amount_in: u64,
    pub amount_out: u64,
    pub fee_amount: u64,
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum SwapStepError {
    #[snafu(display("amount_remaining_without_fee == 0"))]
    AmountRemainingWithoutFeeIsZero,

    #[snafu(display("Liquidity error: {}", source))]
    Liquidity { source: liquidity::LiquidityError },

    #[snafu(display("calculate_fee overflow"))]
    CalculateFeeOverflow,

    #[snafu(display("math overflow"))]
    MathOverflow,

    #[snafu(display("calculate_amount_in_range overflow"))]
    CalculateAmountInRangeOverflow,
}

pub type Result<T> = std::result::Result<T, SwapStepError>;

#[cfg(test)]
mod swap_math_test {
    use proptest::prelude::*;

    use super::*;
    use crate::{constants::FEE_RATE_DENOMINATOR_VALUE, math::tick};

    proptest! {
        #[test]
        fn compute_swap_step_test(
            sqrt_price_current_x64 in tick::MIN_SQRT_PRICE_X64..tick::MAX_SQRT_PRICE_X64,
            sqrt_price_target_x64 in tick::MIN_SQRT_PRICE_X64..tick::MAX_SQRT_PRICE_X64,
            liquidity in 1..u32::MAX as u128,
            amount_remaining in 1..u64::MAX,
            fee_rate in 1..FEE_RATE_DENOMINATOR_VALUE/2,
            is_base_input in proptest::bool::ANY,
        ) {
            prop_assume!(sqrt_price_current_x64 != sqrt_price_target_x64);



            let zero_for_one = sqrt_price_current_x64 > sqrt_price_target_x64;

            let swap_step = compute_swap_step(
                sqrt_price_current_x64,
                sqrt_price_target_x64,
                liquidity,
                amount_remaining,
                fee_rate,
                is_base_input,
                zero_for_one,
            ).expect("compute_swap_step failed");


            let amount_in = swap_step.amount_in;
            let amount_out = swap_step.amount_out;
            let sqrt_price_next_x64 = swap_step.sqrt_price_next_x64;
            let fee_amount = swap_step.fee_amount;

            let amount_used = if is_base_input {
                match amount_in.checked_add(fee_amount) {
                    Some(amount) => amount,
                    None => {
                        println!("amount_in + fee_amount overflow");
                        println!("amount_in: {}", amount_in);
                        println!("fee_amount: {}", fee_amount);
                        amount_out
                    },
                }
            } else {
                amount_out
            };

            if sqrt_price_next_x64 != sqrt_price_target_x64 {
                assert!(amount_used == amount_remaining);
            } else {
                assert!(amount_used <= amount_remaining);
            }
            let price_lower = sqrt_price_current_x64.min(sqrt_price_target_x64);
            let price_upper = sqrt_price_current_x64.max(sqrt_price_target_x64);
            assert!(sqrt_price_next_x64 >= price_lower);
            assert!(sqrt_price_next_x64 <= price_upper);
        }
    }
}
