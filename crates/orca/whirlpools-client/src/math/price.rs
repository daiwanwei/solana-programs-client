use rust_decimal::{prelude::ToPrimitive, Decimal, MathematicalOps};

pub fn calculate_sqrt_price_x64(price: Decimal, decimals_0: u8, decimals_1: u8) -> u128 {
    // Adjust price based on token decimals
    let adjusted_price = if decimals_0 > decimals_1 {
        price * Decimal::from(10u64.pow(decimals_0 as u32 - decimals_1 as u32))
    } else {
        price / Decimal::from(10u64.pow(decimals_1 as u32 - decimals_0 as u32))
    };

    // Calculate square root
    let price_sqrt = adjusted_price.sqrt().unwrap();

    // Convert to Q64 fixed point format
    let price_sqrt_u64 = price_sqrt * Decimal::from(1u128 << 64);
    price_sqrt_u64.to_u128().unwrap()
}

pub fn calculate_price(sqrt_price_x64: u128, decimals_0: u8, decimals_1: u8) -> Decimal {
    let price_sqrt = normalize_price_sqrt(sqrt_price_x64);

    let price = price_sqrt * price_sqrt;

    if decimals_0 > decimals_1 {
        price * Decimal::from(10u64.pow(decimals_0 as u32 - decimals_1 as u32))
    } else {
        price / Decimal::from(10u64.pow(decimals_1 as u32 - decimals_0 as u32))
    }
}

pub fn normalize_price_sqrt(price: u128) -> Decimal {
    Decimal::from(price) / Decimal::from(2u128.pow(64))
}
