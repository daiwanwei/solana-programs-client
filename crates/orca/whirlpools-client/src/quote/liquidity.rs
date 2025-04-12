use ethnum::U256;

use crate::{
    constants::{CoreError, AMOUNT_EXCEEDS_MAX_U64, ARITHMETIC_OVERFLOW},
    math::{
        order_tick_indexes, position_status, tick_index_to_sqrt_price, try_apply_transfer_fee,
        try_get_max_amount_with_slippage_tolerance, try_get_min_amount_with_slippage_tolerance,
        try_reverse_apply_transfer_fee,
    },
    types::{DecreaseLiquidityQuote, IncreaseLiquidityQuote, PositionStatus, TransferFee, U128},
};

/// Calculate the quote for decreasing liquidity
///
/// # Parameters
/// - `liquidity_delta` - The amount of liquidity to decrease
/// - `slippage_tolerance` - The slippage tolerance in bps
/// - `current_sqrt_price` - The current sqrt price of the pool
/// - `tick_index_1` - The first tick index of the position
/// - `tick_index_2` - The second tick index of the position
/// - `transfer_fee_a` - The transfer fee for token A in bps
/// - `transfer_fee_b` - The transfer fee for token B in bps
///
/// # Returns
/// - A DecreaseLiquidityQuote struct containing the estimated token amounts
pub fn decrease_liquidity_quote(
    liquidity_delta: U128,
    slippage_tolerance_bps: u16,
    current_sqrt_price: U128,
    tick_index_1: i32,
    tick_index_2: i32,
    transfer_fee_a: Option<TransferFee>,
    transfer_fee_b: Option<TransferFee>,
) -> Result<DecreaseLiquidityQuote, CoreError> {
    let liquidity_delta: u128 = liquidity_delta.into();
    if liquidity_delta == 0 {
        return Ok(DecreaseLiquidityQuote::default());
    }

    let tick_range = order_tick_indexes(tick_index_1, tick_index_2);
    let current_sqrt_price: u128 = current_sqrt_price.into();

    let (token_est_before_fees_a, token_est_before_fees_b) =
        try_get_token_estimates_from_liquidity(
            liquidity_delta,
            current_sqrt_price,
            tick_range.tick_lower_index,
            tick_range.tick_upper_index,
            false,
        )?;

    let token_est_a =
        try_apply_transfer_fee(token_est_before_fees_a, transfer_fee_a.unwrap_or_default())?;
    let token_est_b =
        try_apply_transfer_fee(token_est_before_fees_b, transfer_fee_b.unwrap_or_default())?;

    let token_min_a =
        try_get_min_amount_with_slippage_tolerance(token_est_a, slippage_tolerance_bps)?;
    let token_min_b =
        try_get_min_amount_with_slippage_tolerance(token_est_b, slippage_tolerance_bps)?;

    Ok(DecreaseLiquidityQuote {
        liquidity_delta,
        token_est_a,
        token_est_b,
        token_min_a,
        token_min_b,
    })
}

/// Calculate the quote for decreasing liquidity given a token a amount
///
/// # Parameters
/// - `token_amount_a` - The amount of token a to decrease
/// - `slippage_tolerance` - The slippage tolerance in bps
/// - `current_sqrt_price` - The current sqrt price of the pool
/// - `tick_index_1` - The first tick index of the position
/// - `tick_index_2` - The second tick index of the position
/// - `transfer_fee_a` - The transfer fee for token A in bps
/// - `transfer_fee_b` - The transfer fee for token B in bps
///
/// # Returns
/// - A DecreaseLiquidityQuote struct containing the estimated token amounts
pub fn decrease_liquidity_quote_a(
    token_amount_a: u64,
    slippage_tolerance_bps: u16,
    current_sqrt_price: U128,
    tick_index_1: i32,
    tick_index_2: i32,
    transfer_fee_a: Option<TransferFee>,
    transfer_fee_b: Option<TransferFee>,
) -> Result<DecreaseLiquidityQuote, CoreError> {
    let tick_range = order_tick_indexes(tick_index_1, tick_index_2);
    let token_delta_a =
        try_reverse_apply_transfer_fee(token_amount_a, transfer_fee_a.unwrap_or_default())?;

    if token_delta_a == 0 {
        return Ok(DecreaseLiquidityQuote::default());
    }

    let current_sqrt_price: u128 = current_sqrt_price.into();
    let sqrt_price_lower: u128 = tick_index_to_sqrt_price(tick_range.tick_lower_index).into();
    let sqrt_price_upper: u128 = tick_index_to_sqrt_price(tick_range.tick_upper_index).into();

    let position_status = position_status(
        current_sqrt_price.into(),
        tick_range.tick_lower_index,
        tick_range.tick_upper_index,
    );

    let liquidity: u128 = match position_status {
        PositionStatus::PriceBelowRange => {
            try_get_liquidity_from_a(token_delta_a, sqrt_price_lower, sqrt_price_upper)?
        }
        PositionStatus::Invalid | PositionStatus::PriceAboveRange => 0,
        PositionStatus::PriceInRange => {
            try_get_liquidity_from_a(token_delta_a, current_sqrt_price, sqrt_price_upper)?
        }
    };

    decrease_liquidity_quote(
        liquidity.into(),
        slippage_tolerance_bps,
        current_sqrt_price.into(),
        tick_index_1,
        tick_index_2,
        transfer_fee_a,
        transfer_fee_b,
    )
}

/// Calculate the quote for decreasing liquidity given a token b amount
///
/// # Parameters
/// - `token_amount_b` - The amount of token b to decrease
/// - `slippage_tolerance` - The slippage tolerance in bps
/// - `current_sqrt_price` - The current sqrt price of the pool
/// - `tick_index_1` - The first tick index of the position
/// - `tick_index_2` - The second tick index of the position
/// - `transfer_fee_a` - The transfer fee for token A in bps
/// - `transfer_fee_b` - The transfer fee for token B in bps
///
/// # Returns
/// - A DecreaseLiquidityQuote struct containing the estimated token amounts
pub fn decrease_liquidity_quote_b(
    token_amount_b: u64,
    slippage_tolerance_bps: u16,
    current_sqrt_price: U128,
    tick_index_1: i32,
    tick_index_2: i32,
    transfer_fee_a: Option<TransferFee>,
    transfer_fee_b: Option<TransferFee>,
) -> Result<DecreaseLiquidityQuote, CoreError> {
    let tick_range = order_tick_indexes(tick_index_1, tick_index_2);
    let token_delta_b =
        try_reverse_apply_transfer_fee(token_amount_b, transfer_fee_b.unwrap_or_default())?;

    if token_delta_b == 0 {
        return Ok(DecreaseLiquidityQuote::default());
    }

    let current_sqrt_price: u128 = current_sqrt_price.into();
    let sqrt_price_lower: u128 = tick_index_to_sqrt_price(tick_range.tick_lower_index).into();
    let sqrt_price_upper: u128 = tick_index_to_sqrt_price(tick_range.tick_upper_index).into();

    let position_status = position_status(
        current_sqrt_price.into(),
        tick_range.tick_lower_index,
        tick_range.tick_upper_index,
    );

    let liquidity: u128 = match position_status {
        PositionStatus::Invalid | PositionStatus::PriceBelowRange => 0,
        PositionStatus::PriceAboveRange => {
            try_get_liquidity_from_b(token_delta_b, sqrt_price_lower, sqrt_price_upper)?
        }
        PositionStatus::PriceInRange => {
            try_get_liquidity_from_b(token_delta_b, sqrt_price_lower, current_sqrt_price)?
        }
    };

    decrease_liquidity_quote(
        liquidity.into(),
        slippage_tolerance_bps,
        current_sqrt_price.into(),
        tick_index_1,
        tick_index_2,
        transfer_fee_a,
        transfer_fee_b,
    )
}

/// Calculate the quote for increasing liquidity
///
/// # Parameters
/// - `liquidity_delta` - The amount of liquidity to increase
/// - `slippage_tolerance` - The slippage tolerance in bps
/// - `current_sqrt_price` - The current sqrt price of the pool
/// - `tick_index_1` - The first tick index of the position
/// - `tick_index_2` - The second tick index of the position
/// - `transfer_fee_a` - The transfer fee for token A in bps
/// - `transfer_fee_b` - The transfer fee for token B in bps
///
/// # Returns
/// - An IncreaseLiquidityQuote struct containing the estimated token amounts
pub fn increase_liquidity_quote(
    liquidity_delta: U128,
    slippage_tolerance_bps: u16,
    current_sqrt_price: U128,
    tick_index_1: i32,
    tick_index_2: i32,
    transfer_fee_a: Option<TransferFee>,
    transfer_fee_b: Option<TransferFee>,
) -> Result<IncreaseLiquidityQuote, CoreError> {
    let liquidity_delta: u128 = liquidity_delta.into();
    if liquidity_delta == 0 {
        return Ok(IncreaseLiquidityQuote::default());
    }

    let current_sqrt_price: u128 = current_sqrt_price.into();
    let tick_range = order_tick_indexes(tick_index_1, tick_index_2);

    let (token_est_before_fees_a, token_est_before_fees_b) =
        try_get_token_estimates_from_liquidity(
            liquidity_delta,
            current_sqrt_price,
            tick_range.tick_lower_index,
            tick_range.tick_upper_index,
            true,
        )?;

    let token_est_a = try_reverse_apply_transfer_fee(
        token_est_before_fees_a,
        transfer_fee_a.unwrap_or_default(),
    )?;
    let token_est_b = try_reverse_apply_transfer_fee(
        token_est_before_fees_b,
        transfer_fee_b.unwrap_or_default(),
    )?;

    let token_max_a =
        try_get_max_amount_with_slippage_tolerance(token_est_a, slippage_tolerance_bps)?;
    let token_max_b =
        try_get_max_amount_with_slippage_tolerance(token_est_b, slippage_tolerance_bps)?;

    Ok(IncreaseLiquidityQuote {
        liquidity_delta,
        token_est_a,
        token_est_b,
        token_max_a,
        token_max_b,
    })
}

/// Calculate the quote for increasing liquidity given a token a amount
///
/// # Parameters
/// - `token_amount_a` - The amount of token a to increase
/// - `slippage_tolerance` - The slippage tolerance in bps
/// - `current_sqrt_price` - The current sqrt price of the pool
/// - `tick_index_1` - The first tick index of the position
/// - `tick_index_2` - The second tick index of the position
/// - `transfer_fee_a` - The transfer fee for token A in bps
/// - `transfer_fee_b` - The transfer fee for token B in bps
///
/// # Returns
/// - An IncreaseLiquidityQuote struct containing the estimated token amounts
pub fn increase_liquidity_quote_a(
    token_amount_a: u64,
    slippage_tolerance_bps: u16,
    current_sqrt_price: U128,
    tick_index_1: i32,
    tick_index_2: i32,
    transfer_fee_a: Option<TransferFee>,
    transfer_fee_b: Option<TransferFee>,
) -> Result<IncreaseLiquidityQuote, CoreError> {
    let tick_range = order_tick_indexes(tick_index_1, tick_index_2);
    let token_delta_a = try_apply_transfer_fee(token_amount_a, transfer_fee_a.unwrap_or_default())?;

    if token_delta_a == 0 {
        return Ok(IncreaseLiquidityQuote::default());
    }

    let current_sqrt_price: u128 = current_sqrt_price.into();
    let sqrt_price_lower: u128 = tick_index_to_sqrt_price(tick_range.tick_lower_index).into();
    let sqrt_price_upper: u128 = tick_index_to_sqrt_price(tick_range.tick_upper_index).into();

    let position_status = position_status(current_sqrt_price.into(), tick_index_1, tick_index_2);

    let liquidity: u128 = match position_status {
        PositionStatus::PriceBelowRange => {
            try_get_liquidity_from_a(token_delta_a, sqrt_price_lower, sqrt_price_upper)?
        }
        PositionStatus::Invalid | PositionStatus::PriceAboveRange => 0,
        PositionStatus::PriceInRange => {
            try_get_liquidity_from_a(token_delta_a, current_sqrt_price, sqrt_price_upper)?
        }
    };

    increase_liquidity_quote(
        liquidity.into(),
        slippage_tolerance_bps,
        current_sqrt_price.into(),
        tick_index_1,
        tick_index_2,
        transfer_fee_a,
        transfer_fee_b,
    )
}

/// Calculate the quote for increasing liquidity given a token b amount
///
/// # Parameters
/// - `token_amount_b` - The amount of token b to increase
/// - `slippage_tolerance` - The slippage tolerance in bps
/// - `current_sqrt_price` - The current sqrt price of the pool
/// - `tick_index_1` - The first tick index of the position
/// - `tick_index_2` - The second tick index of the position
/// - `transfer_fee_a` - The transfer fee for token A in bps
/// - `transfer_fee_b` - The transfer fee for token B in bps
///
/// # Returns
/// - An IncreaseLiquidityQuote struct containing the estimated token amounts
pub fn increase_liquidity_quote_b(
    token_amount_b: u64,
    slippage_tolerance_bps: u16,
    current_sqrt_price: U128,
    tick_index_1: i32,
    tick_index_2: i32,
    transfer_fee_a: Option<TransferFee>,
    transfer_fee_b: Option<TransferFee>,
) -> Result<IncreaseLiquidityQuote, CoreError> {
    let tick_range = order_tick_indexes(tick_index_1, tick_index_2);
    let token_delta_b = try_apply_transfer_fee(token_amount_b, transfer_fee_b.unwrap_or_default())?;

    if token_delta_b == 0 {
        return Ok(IncreaseLiquidityQuote::default());
    }

    let current_sqrt_price: u128 = current_sqrt_price.into();
    let sqrt_price_lower: u128 = tick_index_to_sqrt_price(tick_range.tick_lower_index).into();
    let sqrt_price_upper: u128 = tick_index_to_sqrt_price(tick_range.tick_upper_index).into();

    let position_status = position_status(current_sqrt_price.into(), tick_index_1, tick_index_2);

    let liquidity: u128 = match position_status {
        PositionStatus::Invalid | PositionStatus::PriceBelowRange => 0,
        PositionStatus::PriceAboveRange => {
            try_get_liquidity_from_b(token_delta_b, sqrt_price_lower, sqrt_price_upper)?
        }
        PositionStatus::PriceInRange => {
            try_get_liquidity_from_b(token_delta_b, sqrt_price_lower, current_sqrt_price)?
        }
    };

    increase_liquidity_quote(
        liquidity.into(),
        slippage_tolerance_bps,
        current_sqrt_price.into(),
        tick_index_1,
        tick_index_2,
        transfer_fee_a,
        transfer_fee_b,
    )
}

// Private functions

fn try_get_liquidity_from_a(
    token_delta_a: u64,
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
) -> Result<u128, CoreError> {
    let sqrt_price_diff = sqrt_price_upper - sqrt_price_lower;
    let mul: U256 = <U256>::from(token_delta_a)
        .checked_mul(sqrt_price_lower.into())
        .ok_or(ARITHMETIC_OVERFLOW)?
        .checked_mul(sqrt_price_upper.into())
        .ok_or(ARITHMETIC_OVERFLOW)?;
    let result: U256 = (mul / sqrt_price_diff) >> 64;
    result.try_into().map_err(|_| AMOUNT_EXCEEDS_MAX_U64)
}

fn try_get_token_a_from_liquidity(
    liquidity_delta: u128,
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    round_up: bool,
) -> Result<u64, CoreError> {
    let sqrt_price_diff = sqrt_price_upper - sqrt_price_lower;
    let numerator: U256 = <U256>::from(liquidity_delta)
        .checked_mul(sqrt_price_diff.into())
        .ok_or(ARITHMETIC_OVERFLOW)?
        .checked_shl(64)
        .ok_or(ARITHMETIC_OVERFLOW)?;
    let denominator = <U256>::from(sqrt_price_upper)
        .checked_mul(<U256>::from(sqrt_price_lower))
        .ok_or(ARITHMETIC_OVERFLOW)?;
    let quotient = numerator / denominator;
    let remainder = numerator % denominator;
    if round_up && remainder != 0 {
        (quotient + 1).try_into().map_err(|_| AMOUNT_EXCEEDS_MAX_U64)
    } else {
        quotient.try_into().map_err(|_| AMOUNT_EXCEEDS_MAX_U64)
    }
}

fn try_get_liquidity_from_b(
    token_delta_b: u64,
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
) -> Result<u128, CoreError> {
    let numerator: U256 = <U256>::from(token_delta_b).checked_shl(64).ok_or(ARITHMETIC_OVERFLOW)?;
    let sqrt_price_diff = sqrt_price_upper - sqrt_price_lower;
    let result = numerator / <U256>::from(sqrt_price_diff);
    result.try_into().map_err(|_| AMOUNT_EXCEEDS_MAX_U64)
}

fn try_get_token_b_from_liquidity(
    liquidity_delta: u128,
    sqrt_price_lower: u128,
    sqrt_price_upper: u128,
    round_up: bool,
) -> Result<u64, CoreError> {
    let sqrt_price_diff = sqrt_price_upper - sqrt_price_lower;
    let mul: U256 = <U256>::from(liquidity_delta)
        .checked_mul(sqrt_price_diff.into())
        .ok_or(ARITHMETIC_OVERFLOW)?;
    let result: U256 = mul >> 64;
    if round_up && mul & <U256>::from(u64::MAX) > 0 {
        (result + 1).try_into().map_err(|_| AMOUNT_EXCEEDS_MAX_U64)
    } else {
        result.try_into().map_err(|_| AMOUNT_EXCEEDS_MAX_U64)
    }
}

fn try_get_token_estimates_from_liquidity(
    liquidity_delta: u128,
    current_sqrt_price: u128,
    tick_lower_index: i32,
    tick_upper_index: i32,
    round_up: bool,
) -> Result<(u64, u64), CoreError> {
    if liquidity_delta == 0 {
        return Ok((0, 0));
    }

    let sqrt_price_lower = tick_index_to_sqrt_price(tick_lower_index).into();
    let sqrt_price_upper = tick_index_to_sqrt_price(tick_upper_index).into();

    let position_status =
        position_status(current_sqrt_price.into(), tick_lower_index, tick_upper_index);

    match position_status {
        PositionStatus::PriceBelowRange => {
            let token_a = try_get_token_a_from_liquidity(
                liquidity_delta,
                sqrt_price_lower,
                sqrt_price_upper,
                round_up,
            )?;
            Ok((token_a, 0))
        }
        PositionStatus::PriceInRange => {
            let token_a = try_get_token_a_from_liquidity(
                liquidity_delta,
                current_sqrt_price,
                sqrt_price_upper,
                round_up,
            )?;
            let token_b = try_get_token_b_from_liquidity(
                liquidity_delta,
                sqrt_price_lower,
                current_sqrt_price,
                round_up,
            )?;
            Ok((token_a, token_b))
        }
        PositionStatus::PriceAboveRange => {
            let token_b = try_get_token_b_from_liquidity(
                liquidity_delta,
                sqrt_price_lower,
                sqrt_price_upper,
                round_up,
            )?;
            Ok((0, token_b))
        }
        PositionStatus::Invalid => Ok((0, 0)),
    }
}
