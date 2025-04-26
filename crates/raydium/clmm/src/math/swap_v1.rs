use std::{collections::VecDeque, ops::Neg};

use thiserror::Error;

use crate::{
    constants::FEE_RATE_DENOMINATOR_VALUE,
    generated::{
        accounts::{PoolState, TickArrayBitmapExtension, TickArrayState},
        types::TickState,
    },
    math::{full_math::MulDiv, liquidity, sqrt_price, tick},
};

pub const MAX_SWAP_STEP_COUNT: u32 = 88;

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
    let mut swap_step = SwapStep::default();
    if is_base_input {
        // round up amount_in
        // In exact input case, amount_remaining is positive
        let amount_remaining_less_fee = amount_remaining
            .mul_div_floor(
                (FEE_RATE_DENOMINATOR_VALUE - fee_rate).into(),
                u64::from(FEE_RATE_DENOMINATOR_VALUE),
            )
            .unwrap();

        let amount_in = calculate_amount_in_range(
            sqrt_price_current_x64,
            sqrt_price_target_x64,
            liquidity,
            zero_for_one,
            is_base_input,
        )?;
        if amount_in.is_some() {
            swap_step.amount_in = amount_in.unwrap();
        }

        swap_step.sqrt_price_next_x64 =
            if amount_in.is_some() && amount_remaining_less_fee >= swap_step.amount_in {
                sqrt_price_target_x64
            } else {
                sqrt_price::get_next_sqrt_price_from_input(
                    sqrt_price_current_x64,
                    liquidity,
                    amount_remaining_less_fee,
                    zero_for_one,
                )
            };
    } else {
        let amount_out = calculate_amount_in_range(
            sqrt_price_current_x64,
            sqrt_price_target_x64,
            liquidity,
            zero_for_one,
            is_base_input,
        )?;
        if amount_out.is_some() {
            swap_step.amount_out = amount_out.unwrap();
        }
        // In exact output case, amount_remaining is negative
        swap_step.sqrt_price_next_x64 =
            if amount_out.is_some() && amount_remaining >= swap_step.amount_out {
                sqrt_price_target_x64
            } else {
                sqrt_price::get_next_sqrt_price_from_output(
                    sqrt_price_current_x64,
                    liquidity,
                    amount_remaining,
                    zero_for_one,
                )
            }
    }

    // whether we reached the max possible price for the given ticks
    let max = sqrt_price_target_x64 == swap_step.sqrt_price_next_x64;
    // get the input / output amounts when target price is not reached
    if zero_for_one {
        // if max is reached for exact input case, entire amount_in is needed
        if !(max && is_base_input) {
            swap_step.amount_in = liquidity::get_delta_amount_0_unsigned(
                swap_step.sqrt_price_next_x64,
                sqrt_price_current_x64,
                liquidity,
                true,
            )
            .unwrap();
        };
        // if max is reached for exact output case, entire amount_out is needed
        if !(max && !is_base_input) {
            swap_step.amount_out = liquidity::get_delta_amount_1_unsigned(
                swap_step.sqrt_price_next_x64,
                sqrt_price_current_x64,
                liquidity,
                false,
            )
            .unwrap();
        };
    } else {
        if !(max && is_base_input) {
            swap_step.amount_in = liquidity::get_delta_amount_1_unsigned(
                sqrt_price_current_x64,
                swap_step.sqrt_price_next_x64,
                liquidity,
                true,
            )
            .unwrap();
        };
        if !(max && !is_base_input) {
            swap_step.amount_out = liquidity::get_delta_amount_0_unsigned(
                sqrt_price_current_x64,
                swap_step.sqrt_price_next_x64,
                liquidity,
                false,
            )
            .unwrap();
        };
    }

    // For exact output case, cap the output amount to not exceed the remaining
    // output amount
    if !is_base_input && swap_step.amount_out > amount_remaining {
        swap_step.amount_out = amount_remaining;
    }

    swap_step.fee_amount =
        if is_base_input && swap_step.sqrt_price_next_x64 != sqrt_price_target_x64 {
            // we didn't reach the target, so take the remainder of the maximum input as fee
            // swap dust is granted as fee
            u64::from(amount_remaining).checked_sub(swap_step.amount_in).unwrap()
        } else {
            // take pip percentage as fee
            swap_step
                .amount_in
                .mul_div_ceil(fee_rate.into(), (FEE_RATE_DENOMINATOR_VALUE - fee_rate).into())
                .unwrap()
        };

    Ok(swap_step)
}

/// Computes the result of swapping some amount in, or amount out, given the
/// parameters of the swap

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
        }?;
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
        }?;

        Ok(Some(result))
    }
}

pub fn compute_swap(
    zero_for_one: bool,
    is_base_input: bool,
    is_pool_current_tick_array: bool,
    fee: u32,
    amount_specified: u64,
    current_vaild_tick_array_start_index: i32,
    sqrt_price_limit_x64: u128,
    pool_state: &PoolState,
    tickarray_bitmap_extension: &TickArrayBitmapExtension,
    tick_arrays: &mut VecDeque<TickArrayState>,
) -> Result<(SwapState, VecDeque<i32>)> {
    // Input validation
    if amount_specified == 0 {
        return Err(SwapError::AmountSpecifiedZero);
    }

    // Set price limit
    let sqrt_price_limit_x64 = if sqrt_price_limit_x64 == 0 {
        if zero_for_one {
            tick::MIN_SQRT_PRICE_X64 + 1
        } else {
            tick::MAX_SQRT_PRICE_X64 - 1
        }
    } else {
        sqrt_price_limit_x64
    };

    // Validate price limits
    match zero_for_one {
        true if sqrt_price_limit_x64 < tick::MIN_SQRT_PRICE_X64 => {
            return Err(SwapError::SqrtPriceLimitX64TooSmall)
        }
        true if sqrt_price_limit_x64 >= pool_state.sqrt_price_x64 => {
            return Err(SwapError::SqrtPriceLimitX64TooLarge)
        }
        false if sqrt_price_limit_x64 > tick::MAX_SQRT_PRICE_X64 => {
            return Err(SwapError::SqrtPriceLimitX64TooLarge)
        }
        false if sqrt_price_limit_x64 <= pool_state.sqrt_price_x64 => {
            return Err(SwapError::SqrtPriceLimitX64TooSmall)
        }
        _ => {}
    }

    // Initialize state
    let mut tick_match_current_tick_array = is_pool_current_tick_array;
    let mut state = SwapState {
        amount_specified_remaining: amount_specified,
        amount_calculated: 0,
        sqrt_price_x64: pool_state.sqrt_price_x64,
        tick: pool_state.tick_current,
        liquidity: pool_state.liquidity,
    };

    // Setup initial tick array
    let mut tick_array_current = tick_arrays.pop_front().ok_or(SwapError::NoTickArrayAvailable)?;
    if tick_array_current.start_tick_index != current_vaild_tick_array_start_index {
        return Err(SwapError::TickArrayStartTickIndexDoesNotMatch);
    }

    let mut tick_array_indices = VecDeque::new();
    tick_array_indices.push_back(tick_array_current.start_tick_index);

    // Main swap loop
    let mut loop_count = 0;
    while state.amount_specified_remaining != 0
        && state.sqrt_price_x64 != sqrt_price_limit_x64
        && state.tick.clamp(tick::MIN_TICK, tick::MAX_TICK) == state.tick
    {
        if loop_count > MAX_SWAP_STEP_COUNT {
            return Err(SwapError::LoopCountLimit);
        }

        let mut step = StepComputations::default();
        step.sqrt_price_start_x64 = state.sqrt_price_x64;

        // Get next initialized tick
        let mut next_tick = process_next_tick(
            &mut tick_array_current,
            &mut tick_match_current_tick_array,
            state.tick,
            pool_state,
            zero_for_one,
        )?;

        // Handle uninitialized tick
        if !next_tick.is_initialized() {
            (tick_array_current, next_tick) = handle_uninitialized_tick(
                tick_arrays,
                pool_state,
                tickarray_bitmap_extension,
                current_vaild_tick_array_start_index,
                zero_for_one,
                &mut tick_array_indices,
            )?;
        }

        // Calculate next price and swap step
        step.tick_next = next_tick.tick.clamp(tick::MIN_TICK, tick::MAX_TICK);
        step.initialized = next_tick.is_initialized();
        step.sqrt_price_next_x64 = tick::get_sqrt_price_at_tick(step.tick_next)?;

        let target_price =
            calculate_target_price(zero_for_one, step.sqrt_price_next_x64, sqrt_price_limit_x64);

        // Compute and apply swap step
        let swap_result = compute_swap_step(
            state.sqrt_price_x64,
            target_price,
            state.liquidity,
            state.amount_specified_remaining,
            fee,
            is_base_input,
            zero_for_one,
        )?;

        // Update state with swap results
        update_state_from_swap(
            &mut state,
            &swap_result,
            &mut step,
            is_base_input,
            zero_for_one,
            &next_tick,
        )?;

        loop_count += 1;
    }

    Ok((state, tick_array_indices))
}

pub fn compute_swap_by_specified_sqrt_price(
    is_pool_current_tick_array: bool,
    fee: u32,
    current_vaild_tick_array_start_index: i32,
    sqrt_price: u128,
    pool_state: &PoolState,
    tickarray_bitmap_extension: &TickArrayBitmapExtension,
    tick_arrays: &mut VecDeque<TickArrayState>,
) -> Result<(SwapState, VecDeque<i32>)> {
    if sqrt_price == pool_state.sqrt_price_x64 {
        return Ok((
            SwapState {
                amount_specified_remaining: 0,
                amount_calculated: 0,
                sqrt_price_x64: pool_state.sqrt_price_x64,
                tick: pool_state.tick_current,
                liquidity: pool_state.liquidity,
            },
            VecDeque::new(),
        ));
    }

    const IS_BASE_INPUT: bool = true;

    let zero_for_one = sqrt_price < pool_state.sqrt_price_x64;

    // Check if target price is valid based on swap direction
    if zero_for_one {
        if sqrt_price < tick::MIN_SQRT_PRICE_X64 {
            return Err(SwapError::SqrtPriceLimitX64TooSmall);
        }
    } else {
        if sqrt_price > tick::MAX_SQRT_PRICE_X64 {
            return Err(SwapError::SqrtPriceLimitX64TooLarge);
        }
    }

    // Initialize swap state with maximum amount for price-based swap
    let mut state = SwapState {
        amount_specified_remaining: u64::MAX,
        amount_calculated: 0,
        sqrt_price_x64: pool_state.sqrt_price_x64,
        tick: pool_state.tick_current,
        liquidity: pool_state.liquidity,
    };

    // Initialize tick array tracking
    let mut tick_array_current = tick_arrays.pop_front().ok_or(SwapError::NoTickArrayAvailable)?;

    if tick_array_current.start_tick_index != current_vaild_tick_array_start_index {
        return Err(SwapError::TickArrayStartTickIndexDoesNotMatch);
    }

    let mut tick_array_indices = VecDeque::from([tick_array_current.start_tick_index]);
    let mut tick_match_current_tick_array = is_pool_current_tick_array;

    // Main swap loop - continue until target price is reached
    for loop_count in 0..MAX_SWAP_STEP_COUNT + 1 {
        if state.sqrt_price_x64 == sqrt_price
            || state.tick.clamp(tick::MIN_TICK, tick::MAX_TICK) != state.tick
        {
            break;
        }
        if loop_count == MAX_SWAP_STEP_COUNT {
            return Err(SwapError::LoopCountLimit);
        }
        // Process current step
        let mut step =
            StepComputations { sqrt_price_start_x64: state.sqrt_price_x64, ..Default::default() };

        // Get and process next tick
        let mut next_tick = process_next_tick(
            &mut tick_array_current,
            &mut tick_match_current_tick_array,
            state.tick,
            pool_state,
            zero_for_one,
        )?;

        // Handle case when tick is not initialized
        if !next_tick.is_initialized() {
            (tick_array_current, next_tick) = handle_uninitialized_tick(
                tick_arrays,
                pool_state,
                tickarray_bitmap_extension,
                current_vaild_tick_array_start_index,
                zero_for_one,
                &mut tick_array_indices,
            )?;
        }

        // Calculate prices and execute swap step
        step.tick_next = next_tick.tick.clamp(tick::MIN_TICK, tick::MAX_TICK);
        step.initialized = next_tick.is_initialized();
        step.sqrt_price_next_x64 = tick::get_sqrt_price_at_tick(step.tick_next)?;

        let target_price =
            calculate_target_price(zero_for_one, step.sqrt_price_next_x64, sqrt_price);
        // Execute swap step and update state
        let swap_result = compute_swap_step(
            state.sqrt_price_x64,
            target_price,
            state.liquidity,
            state.amount_specified_remaining,
            fee,
            IS_BASE_INPUT,
            zero_for_one,
        )?;
        println!("swap_result: {:?}", swap_result);

        update_state_from_swap(
            &mut state,
            &swap_result,
            &mut step,
            IS_BASE_INPUT,
            zero_for_one,
            &next_tick,
        )?;
        println!("state: {:?}", state);
    }

    Ok((state, tick_array_indices))
}

// Helper functions to break down the complexity
fn process_next_tick(
    tick_array: &TickArrayState,
    tick_match_current: &mut bool,
    current_tick: i32,
    pool_state: &PoolState,
    zero_for_one: bool,
) -> Result<Box<TickState>> {
    match tick_array
        .next_initialized_tick(current_tick, pool_state.tick_spacing, zero_for_one)
        .map_err(|_| SwapError::TickState)?
    {
        Some(tick_state) => Ok(Box::new(tick_state)),
        None if !*tick_match_current => {
            *tick_match_current = true;
            Ok(Box::new(
                tick_array
                    .first_initialized_tick(zero_for_one)
                    .map_err(|_| SwapError::TickState)?,
            ))
        }
        None => Ok(Box::new(TickState {
            tick: 0,
            liquidity_net: 0,
            liquidity_gross: 0,
            fee_growth_outside0_x64: 0,
            fee_growth_outside1_x64: 0,
            reward_growths_outside_x64: [0; 3],
            padding: [0; 13],
        })),
    }
}

fn handle_uninitialized_tick(
    tick_arrays: &mut VecDeque<TickArrayState>,
    pool_state: &PoolState,
    bitmap_extension: &TickArrayBitmapExtension,
    current_index: i32,
    zero_for_one: bool,
    indices: &mut VecDeque<i32>,
) -> Result<(TickArrayState, Box<TickState>)> {
    let next_index = pool_state
        .next_initialized_tick_array_start_index(
            &Some(bitmap_extension.clone()),
            current_index,
            zero_for_one,
        )
        .map_err(|_| SwapError::PoolState)?
        .ok_or(SwapError::TickArrayStartTickIndexOutOfRangeLimit)?;

    let next_array = tick_arrays.pop_front().ok_or(SwapError::NoMoreTickArraysAvailable)?;

    if next_array.start_tick_index != next_index {
        return Err(SwapError::TickArrayStartTickIndexDoesNotMatch);
    }

    indices.push_back(next_array.start_tick_index);
    let first_tick =
        next_array.first_initialized_tick(zero_for_one).map_err(|_| SwapError::TickState)?;

    Ok((next_array, Box::new(first_tick)))
}

fn calculate_target_price(zero_for_one: bool, next_price: u128, limit_price: u128) -> u128 {
    if (zero_for_one && next_price < limit_price) || (!zero_for_one && next_price > limit_price) {
        limit_price
    } else {
        next_price
    }
}

fn update_state_from_swap(
    state: &mut SwapState,
    swap_result: &SwapStep,
    step: &mut StepComputations,
    is_base_input: bool,
    zero_for_one: bool,
    next_tick: &TickState,
) -> Result<()> {
    state.sqrt_price_x64 = swap_result.sqrt_price_next_x64;
    step.amount_in = swap_result.amount_in;
    step.amount_out = swap_result.amount_out;
    step.fee_amount = swap_result.fee_amount;

    // Update amounts
    if is_base_input {
        state.amount_specified_remaining = state
            .amount_specified_remaining
            .checked_sub(step.amount_in + step.fee_amount)
            .ok_or(SwapError::MathOverflow)?;
        state.amount_calculated =
            state.amount_calculated.checked_add(step.amount_out).ok_or(SwapError::MathOverflow)?;
    } else {
        state.amount_specified_remaining = state
            .amount_specified_remaining
            .checked_sub(step.amount_out)
            .ok_or(SwapError::MathOverflow)?;
        state.amount_calculated = state
            .amount_calculated
            .checked_add(step.amount_in + step.fee_amount)
            .ok_or(SwapError::MathOverflow)?;
    }

    // Update tick and liquidity
    if state.sqrt_price_x64 == step.sqrt_price_next_x64 {
        if step.initialized {
            let liquidity_net =
                if zero_for_one { next_tick.liquidity_net.neg() } else { next_tick.liquidity_net };
            state.liquidity = liquidity::add_delta(state.liquidity, liquidity_net)?;
        }
        state.tick = if zero_for_one { step.tick_next - 1 } else { step.tick_next };
    } else if state.sqrt_price_x64 != step.sqrt_price_start_x64 {
        state.tick = tick::get_tick_at_sqrt_price(state.sqrt_price_x64)?;
    }

    Ok(())
}

// the top level state of the swap, the results of which are recorded in storage
// at the end
#[derive(Debug)]
pub struct SwapState {
    // the amount remaining to be swapped in/out of the input/output asset
    pub amount_specified_remaining: u64,
    // the amount already swapped out/in of the output/input asset
    pub amount_calculated: u64,
    // current sqrt(price)
    pub sqrt_price_x64: u128,
    // the tick associated with the current price
    pub tick: i32,
    // the current liquidity in range
    pub liquidity: u128,
}

#[derive(Default)]
struct StepComputations {
    // the price at the beginning of the step
    sqrt_price_start_x64: u128,
    // the next tick to swap to from the current tick in the swap direction
    tick_next: i32,
    // whether tick_next is initialized or not
    initialized: bool,
    // sqrt(price) for the next tick (1/0)
    sqrt_price_next_x64: u128,
    // how much is being swapped in in this step
    amount_in: u64,
    // how much is being swapped out
    amount_out: u64,
    // how much fee is being paid in
    fee_amount: u64,
}

#[derive(Debug, Error)]
pub enum SwapError {
    #[error("Liquidity error: {0}")]
    Liquidity(#[from] liquidity::LiquidityError),

    #[error("Tick error: {0}")]
    Tick(#[from] tick::TickError),

    #[error("Tick state error")]
    TickState,

    #[error("Pool state error")]
    PoolState,

    #[error("Math overflow")]
    MathOverflow,

    #[error("Amount specified zero")]
    AmountSpecifiedZero,

    #[error("Sqrt price limit x64 too small")]
    SqrtPriceLimitX64TooSmall,

    #[error("Sqrt price limit x64 too large")]
    SqrtPriceLimitX64TooLarge,

    #[error("Tick array start tick index does not match")]
    TickArrayStartTickIndexDoesNotMatch,

    #[error("Tick array start tick index out of range limit")]
    TickArrayStartTickIndexOutOfRangeLimit,

    #[error("Loop count limit")]
    LoopCountLimit,

    #[error("No tick array available")]
    NoTickArrayAvailable,

    #[error("No more tick arrays available")]
    NoMoreTickArraysAvailable,

    #[error("Tick state not initialized")]
    TickStateNotInitialized,
}

pub type Result<T> = std::result::Result<T, SwapError>;
