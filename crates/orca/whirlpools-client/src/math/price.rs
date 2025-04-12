use libm::{floor, pow, sqrt};

use super::{invert_tick_index, sqrt_price_to_tick_index, tick_index_to_sqrt_price};
use crate::types::U128;

const Q64_RESOLUTION: f64 = 18446744073709551616.0;

/// Convert a price into a sqrt priceX64
/// IMPORTANT: floating point operations can reduce the precision of the result.
/// Make sure to do these operations last and not to use the result for further
/// calculations.
///
/// # Parameters
/// * `price` - The price to convert
/// * `decimals_a` - The number of decimals of the base token
/// * `decimals_b` - The number of decimals of the quote token
///
/// # Returns
/// * `u128` - The sqrt priceX64
pub fn price_to_sqrt_price(price: f64, decimals_a: u8, decimals_b: u8) -> U128 {
    let power = pow(10f64, decimals_a as f64 - decimals_b as f64);
    (floor(sqrt(price / power) * Q64_RESOLUTION) as u128).into()
}

/// Convert a sqrt priceX64 into a tick index
/// IMPORTANT: floating point operations can reduce the precision of the result.
/// Make sure to do these operations last and not to use the result for further
/// calculations.
///
/// # Parameters
/// * `sqrt_price` - The sqrt priceX64 to convert
/// * `decimals_a` - The number of decimals of the base token
/// * `decimals_b` - The number of decimals of the quote token
///
/// # Returns
/// * `f64` - The decimal price
pub fn sqrt_price_to_price(sqrt_price: U128, decimals_a: u8, decimals_b: u8) -> f64 {
    let power = pow(10f64, decimals_a as f64 - decimals_b as f64);
    let sqrt_price: u128 = sqrt_price.into();
    let sqrt_price_u128 = sqrt_price as f64;
    pow(sqrt_price_u128 / Q64_RESOLUTION, 2.0) * power
}

/// Invert a price
/// IMPORTANT: floating point operations can reduce the precision of the result.
/// Make sure to do these operations last and not to use the result for further
/// calculations.
///
/// # Parameters
/// * `price` - The price to invert
/// * `decimals_a` - The number of decimals of the base token
/// * `decimals_b` - The number of decimals of the quote token
///
/// # Returns
/// * `f64` - The inverted price
pub fn invert_price(price: f64, decimals_a: u8, decimals_b: u8) -> f64 {
    let tick_index = price_to_tick_index(price, decimals_a, decimals_b);
    let inverted_tick_index = invert_tick_index(tick_index);
    tick_index_to_price(inverted_tick_index, decimals_a, decimals_b)
}

/// Convert a tick index into a price
/// IMPORTANT: floating point operations can reduce the precision of the result.
/// Make sure to do these operations last and not to use the result for further
/// calculations.
///
/// # Parameters
/// * `tick_index` - The tick index to convert
/// * `decimals_a` - The number of decimals of the base token
/// * `decimals_b` - The number of decimals of the quote token
///
/// # Returns
/// * `f64` - The decimal price
pub fn tick_index_to_price(tick_index: i32, decimals_a: u8, decimals_b: u8) -> f64 {
    let sqrt_price = tick_index_to_sqrt_price(tick_index);
    sqrt_price_to_price(sqrt_price, decimals_a, decimals_b)
}

/// Convert a price into a tick index
/// IMPORTANT: floating point operations can reduce the precision of the result.
/// Make sure to do these operations last and not to use the result for further
/// calculations.
///
/// # Parameters
/// * `price` - The price to convert
/// * `decimals_a` - The number of decimals of the base token
/// * `decimals_b` - The number of decimals of the quote token
///
/// # Returns
/// * `i32` - The tick index
pub fn price_to_tick_index(price: f64, decimals_a: u8, decimals_b: u8) -> i32 {
    let sqrt_price = price_to_sqrt_price(price, decimals_a, decimals_b);
    sqrt_price_to_tick_index(sqrt_price)
}
