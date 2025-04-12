use ethnum::U256;

use super::{order_tick_indexes, tick_index_to_sqrt_price};
use crate::{
    constants::BPS_DENOMINATOR,
    types::{PositionRatio, PositionStatus, U128},
};

/// Check if a position is in range.
/// When a position is in range it is earning fees and rewards
///
/// # Parameters
/// - `sqrt_price` - A u128 integer representing the sqrt price of the pool
/// - `tick_index_1` - A i32 integer representing the first tick index of the
///   position
/// - `tick_index_2` - A i32 integer representing the second tick index of the
///   position
///
/// # Returns
/// - A boolean value indicating if the position is in range
pub fn is_position_in_range(
    current_sqrt_price: U128,
    tick_index_1: i32,
    tick_index_2: i32,
) -> bool {
    position_status(current_sqrt_price.into(), tick_index_1, tick_index_2)
        == PositionStatus::PriceInRange
}

/// Calculate the status of a position
/// The status can be one of three values:
/// - InRange: The position is in range
/// - BelowRange: The position is below the range
/// - AboveRange: The position is above the range
///
/// # Parameters
/// - `sqrt_price` - A u128 integer representing the sqrt price of the pool
/// - `tick_index_1` - A i32 integer representing the first tick index of the
///   position
/// - `tick_index_2` - A i32 integer representing the second tick index of the
///   position
///
/// # Returns
/// - A PositionStatus enum value indicating the status of the position
pub fn position_status(
    current_sqrt_price: U128,
    tick_index_1: i32,
    tick_index_2: i32,
) -> PositionStatus {
    let current_sqrt_price: u128 = current_sqrt_price.into();
    let tick_range = order_tick_indexes(tick_index_1, tick_index_2);
    let sqrt_price_lower: u128 = tick_index_to_sqrt_price(tick_range.tick_lower_index).into();
    let sqrt_price_upper: u128 = tick_index_to_sqrt_price(tick_range.tick_upper_index).into();

    if tick_index_1 == tick_index_2 {
        PositionStatus::Invalid
    } else if current_sqrt_price <= sqrt_price_lower {
        PositionStatus::PriceBelowRange
    } else if current_sqrt_price >= sqrt_price_upper {
        PositionStatus::PriceAboveRange
    } else {
        PositionStatus::PriceInRange
    }
}

/// Calculate the token_a / token_b ratio of a (ficticious) position
///
/// # Parameters
/// - `sqrt_price` - A u128 integer representing the sqrt price of the pool
/// - `tick_index_1` - A i32 integer representing the first tick index of the
///   position
/// - `tick_index_2` - A i32 integer representing the second tick index of the
///   position
///
/// # Returns
/// - A PositionRatio struct containing the ratio of token_a and token_b
pub fn position_ratio(
    current_sqrt_price: U128,
    tick_index_1: i32,
    tick_index_2: i32,
) -> PositionRatio {
    let current_sqrt_price: u128 = current_sqrt_price.into();
    let position_status = position_status(current_sqrt_price.into(), tick_index_1, tick_index_2);
    match position_status {
        PositionStatus::Invalid => PositionRatio { ratio_a: 0, ratio_b: 0 },
        PositionStatus::PriceBelowRange => PositionRatio { ratio_a: 10000, ratio_b: 0 },
        PositionStatus::PriceAboveRange => PositionRatio { ratio_a: 0, ratio_b: 10000 },
        PositionStatus::PriceInRange => {
            let tick_range = order_tick_indexes(tick_index_1, tick_index_2);
            let lower_sqrt_price: u128 =
                tick_index_to_sqrt_price(tick_range.tick_lower_index).into();
            let upper_sqrt_price: u128 =
                tick_index_to_sqrt_price(tick_range.tick_upper_index).into();

            let l: U256 = <U256>::from(1u16) << 128;
            let p = <U256>::from(current_sqrt_price) * <U256>::from(current_sqrt_price);

            let deposit_a_1: U256 = (l << 64) / current_sqrt_price;
            let deposit_a_2: U256 = (l << 64) / upper_sqrt_price;
            let deposit_a: U256 = ((deposit_a_1 - deposit_a_2) * p) >> 128;

            let deposit_b_1 = current_sqrt_price - lower_sqrt_price;
            let deposit_b = (l * deposit_b_1) >> 64;

            let total_deposit = deposit_a + deposit_b;

            let denominator = <U256>::from(BPS_DENOMINATOR);
            let ratio_a: U256 = (deposit_a * denominator) / total_deposit;
            let ratio_b: U256 = denominator - ratio_a;

            PositionRatio { ratio_a: ratio_a.as_u16(), ratio_b: ratio_b.as_u16() }
        }
    }
}
