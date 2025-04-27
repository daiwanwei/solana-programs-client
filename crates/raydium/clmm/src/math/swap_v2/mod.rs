pub mod swap_step;

use std::{collections::VecDeque, ops::Neg};

use thiserror::Error;

use crate::{
    generated::{
        accounts::{PoolState, TickArrayBitmapExtension, TickArrayState},
        types::TickState,
    },
    math::{liquidity, tick},
};

pub const MAX_SWAP_STEP_COUNT: u32 = 88;

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

    let mut cache = SwapCache {
        sqrt_price_x64: pool_state.sqrt_price_x64,
        tick: pool_state.tick_current,
        liquidity: pool_state.liquidity,
        initialized: true,
        amount_in: 0,
        amount_out: 0,
        fee_amount: 0,
        remaining_amount: amount_specified,
        amount_calculated: 0,
    };

    // Setup initial tick array
    let mut tick_array_current = tick_arrays.pop_front().ok_or(SwapError::NoTickArrayAvailable)?;

    if tick_array_current.start_tick_index != current_vaild_tick_array_start_index {
        return Err(SwapError::TickArrayStartTickIndexDoesNotMatch);
    }

    let mut tick_array_indices = VecDeque::new();
    tick_array_indices.push_back(tick_array_current.start_tick_index);

    let mut current_tick_start = current_vaild_tick_array_start_index;

    // Main swap loop - continue until target price is reached
    for loop_count in 0..MAX_SWAP_STEP_COUNT + 1 {
        if cache.sqrt_price_x64 == sqrt_price_limit_x64
            || cache.tick.clamp(tick::MIN_TICK, tick::MAX_TICK) != cache.tick
        {
            break;
        }

        if cache.remaining_amount == 0 {
            break;
        }

        if loop_count == MAX_SWAP_STEP_COUNT {
            return Err(SwapError::LoopCountLimit);
        }

        // Get and process next tick
        let mut next_tick_state = process_next_tick(
            &mut tick_array_current,
            &mut tick_match_current_tick_array,
            cache.tick,
            pool_state,
            zero_for_one,
        )?;

        // Handle case when tick is not initialized
        if !next_tick_state.is_initialized() {
            (tick_array_current, next_tick_state, current_tick_start) = handle_uninitialized_tick(
                tick_arrays,
                pool_state,
                tickarray_bitmap_extension,
                current_tick_start,
                zero_for_one,
                &mut tick_array_indices,
            )?;
        }

        // Calculate prices and execute swap step
        let next_tick = next_tick_state.tick.clamp(tick::MIN_TICK, tick::MAX_TICK);
        let next_sqrt_price_x64 = tick::get_sqrt_price_at_tick(next_tick)?;

        if next_sqrt_price_x64 == cache.sqrt_price_x64 {
            return Err(SwapError::InvalidNextSqrtPriceX64);
        }

        let target_price =
            calculate_target_price(zero_for_one, next_sqrt_price_x64, sqrt_price_limit_x64);
        // Execute swap step and update state
        let swap_result = swap_step::compute_swap_step(
            cache.sqrt_price_x64,
            target_price,
            cache.liquidity,
            cache.remaining_amount,
            fee,
            is_base_input,
            zero_for_one,
        )?;
        update_cache_from_swap_step(
            &mut cache,
            &swap_result,
            zero_for_one,
            &next_tick_state,
            is_base_input,
        )?;
    }

    let state = SwapState {
        amount_in: cache.amount_in,
        amount_out: cache.amount_out,
        sqrt_price_x64: cache.sqrt_price_x64,
        tick: cache.tick,
        liquidity: cache.liquidity,
        remaining_amount: cache.remaining_amount,
        amount_calculated: cache.amount_calculated,
    };
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
                amount_in: 0,
                amount_out: 0,
                sqrt_price_x64: pool_state.sqrt_price_x64,
                tick: pool_state.tick_current,
                liquidity: pool_state.liquidity,
                remaining_amount: u64::MAX,
                amount_calculated: 0,
            },
            VecDeque::new(),
        ));
    }

    const IS_BASE_INPUT: bool = true;
    let zero_for_one = sqrt_price < pool_state.sqrt_price_x64;

    // Check if target price is valid based on swap direction
    let _ = check_sqrt_price_limit(sqrt_price, zero_for_one)?;

    // Initialize tick array tracking
    let mut tick_array_current = tick_arrays.pop_front().ok_or(SwapError::NoTickArrayAvailable)?;

    if tick_array_current.start_tick_index != current_vaild_tick_array_start_index {
        return Err(SwapError::TickArrayStartTickIndexDoesNotMatch);
    }

    let mut tick_array_indices = VecDeque::from([tick_array_current.start_tick_index]);
    let mut tick_match_current_tick_array = is_pool_current_tick_array;

    let mut cache = SwapCache {
        sqrt_price_x64: pool_state.sqrt_price_x64,
        tick: pool_state.tick_current,
        liquidity: pool_state.liquidity,
        amount_in: 0,
        amount_out: 0,
        fee_amount: 0,
        remaining_amount: u64::MAX,
        initialized: true,
        amount_calculated: 0,
    };

    let mut current_tick_start = current_vaild_tick_array_start_index;

    // Main swap loop - continue until target price is reached
    for loop_count in 0..MAX_SWAP_STEP_COUNT + 1 {
        if cache.sqrt_price_x64 == sqrt_price
            || cache.tick.clamp(tick::MIN_TICK, tick::MAX_TICK) != cache.tick
        {
            break;
        }

        if cache.remaining_amount == 0 {
            break;
        }

        if loop_count == MAX_SWAP_STEP_COUNT {
            return Err(SwapError::LoopCountLimit);
        }
        // Get and process next tick
        let mut next_tick_state = process_next_tick(
            &mut tick_array_current,
            &mut tick_match_current_tick_array,
            cache.tick,
            pool_state,
            zero_for_one,
        )?;

        // Handle case when tick is not initialized
        if !next_tick_state.is_initialized() {
            (tick_array_current, next_tick_state, current_tick_start) = handle_uninitialized_tick(
                tick_arrays,
                pool_state,
                tickarray_bitmap_extension,
                current_tick_start,
                zero_for_one,
                &mut tick_array_indices,
            )?;
        }

        // Calculate prices and execute swap step
        let next_tick = next_tick_state.tick.clamp(tick::MIN_TICK, tick::MAX_TICK);
        let next_sqrt_price_x64 = tick::get_sqrt_price_at_tick(next_tick)?;

        let target_price = calculate_target_price(zero_for_one, next_sqrt_price_x64, sqrt_price);
        // Execute swap step and update state
        let swap_result = swap_step::compute_swap_step(
            cache.sqrt_price_x64,
            target_price,
            cache.liquidity,
            cache.remaining_amount,
            fee,
            IS_BASE_INPUT,
            zero_for_one,
        )?;

        update_cache_from_swap_step(
            &mut cache,
            &swap_result,
            zero_for_one,
            &next_tick_state,
            IS_BASE_INPUT,
        )?;
    }

    let state = SwapState {
        amount_in: cache.amount_in,
        amount_out: cache.amount_out,
        sqrt_price_x64: cache.sqrt_price_x64,
        tick: cache.tick,
        liquidity: cache.liquidity,
        remaining_amount: cache.remaining_amount,
        amount_calculated: cache.amount_calculated,
    };
    println!("state: {:?}", state);

    Ok((state, tick_array_indices))
}

fn check_sqrt_price_limit(sqrt_price: u128, zero_for_one: bool) -> Result<()> {
    if zero_for_one {
        if sqrt_price < tick::MIN_SQRT_PRICE_X64 {
            return Err(SwapError::SqrtPriceLimitX64TooSmall);
        }
    } else {
        if sqrt_price > tick::MAX_SQRT_PRICE_X64 {
            return Err(SwapError::SqrtPriceLimitX64TooLarge);
        }
    }
    Ok(())
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
) -> Result<(TickArrayState, Box<TickState>, i32)> {
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

    Ok((next_array, Box::new(first_tick), next_index))
}

fn calculate_target_price(zero_for_one: bool, next_price: u128, limit_price: u128) -> u128 {
    if (zero_for_one && next_price < limit_price) || (!zero_for_one && next_price > limit_price) {
        limit_price
    } else {
        next_price
    }
}

fn update_cache_from_swap_step(
    cache: &mut SwapCache,
    step: &swap_step::SwapStep,
    zero_for_one: bool,
    next_tick: &TickState,
    is_base_input: bool,
) -> Result<()> {
    cache.tick = next_tick.tick;
    cache.initialized = next_tick.is_initialized();
    cache.sqrt_price_x64 = step.sqrt_price_next_x64;

    cache.amount_in = cache.amount_in.checked_add(step.amount_in).ok_or(SwapError::MathOverflow)?;
    cache.amount_out =
        cache.amount_out.checked_add(step.amount_out).ok_or(SwapError::MathOverflow)?;

    cache.fee_amount =
        cache.fee_amount.checked_add(step.fee_amount).ok_or(SwapError::MathOverflow)?;

    // Update tick and liquidity

    if cache.initialized {
        let liquidity_net =
            if zero_for_one { next_tick.liquidity_net.neg() } else { next_tick.liquidity_net };
        cache.liquidity = liquidity::add_delta(cache.liquidity, liquidity_net)?;
    }

    if is_base_input {
        cache.remaining_amount =
            cache.remaining_amount.checked_sub(step.amount_in).ok_or(SwapError::MathOverflow)?;
        cache.amount_calculated =
            cache.amount_calculated.checked_add(step.amount_out).ok_or(SwapError::MathOverflow)?;
    } else {
        cache.remaining_amount =
            cache.remaining_amount.checked_sub(step.amount_out).ok_or(SwapError::MathOverflow)?;
        cache.amount_calculated =
            cache.amount_calculated.checked_add(step.amount_in).ok_or(SwapError::MathOverflow)?;
    }

    Ok(())
}

#[derive(Clone, Debug, Default)]
pub struct SwapState {
    pub amount_in: u64,
    pub amount_out: u64,
    pub sqrt_price_x64: u128,
    pub tick: i32,
    pub liquidity: u128,
    pub remaining_amount: u64,
    pub amount_calculated: u64,
}

#[derive(Default, Debug)]
struct SwapCache {
    // the price at the beginning of the step
    sqrt_price_x64: u128,
    // the tick associated with the current price
    tick: i32,
    // whether tick_next is initialized or not
    initialized: bool,
    // the current liquidity in range
    liquidity: u128,
    // how much is being swapped in in this step
    amount_in: u64,
    // how much is being swapped out
    amount_out: u64,
    // how much fee is being paid in
    fee_amount: u64,
    // the remaining amount to be swapped
    remaining_amount: u64,
    // the amount calculated
    amount_calculated: u64,
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

    #[error("Swap step error: {0}")]
    SwapStep(#[from] swap_step::SwapStepError),

    #[error("Invalid next sqrt price x64")]
    InvalidNextSqrtPriceX64,
}

pub type Result<T> = std::result::Result<T, SwapError>;
