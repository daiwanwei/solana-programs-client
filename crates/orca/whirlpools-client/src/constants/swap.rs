#![allow(non_snake_case)]

/// The denominator of the fee rate value.
pub const FEE_RATE_DENOMINATOR: u32 = 1_000_000;

// TODO: WASM export (which doesn't work with u128 yet)

/// The minimum sqrt price for a whirlpool.
pub const MIN_SQRT_PRICE: u128 = 4295048016;

/// The maximum sqrt price for a whirlpool.
pub const MAX_SQRT_PRICE: u128 = 79226673515401279992447579055;
