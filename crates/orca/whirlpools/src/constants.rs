pub const NUM_REWARDS: usize = 3;

// Max & min tick index based on sqrt(1.0001) & max.min price of 2^64
pub const MAX_TICK_INDEX: i32 = 443636;
pub const MIN_TICK_INDEX: i32 = -443636;

// We have two consts because most of our code uses it as a i32. However,
// for us to use it in tick array declarations, anchor requires it to be a
// usize.
pub const TICK_ARRAY_SIZE: i32 = 88;
pub const TICK_ARRAY_SIZE_USIZE: usize = 88;

// Fee rate is represented as hundredths of a basis point.
// Fee amount = total_amount * fee_rate / 1_000_000.
// Max fee rate supported is 3%.
pub const MAX_FEE_RATE: u16 = 30_000;

// Assuming that FEE_RATE is represented as hundredths of a basis point
// We want FEE_RATE_MUL_VALUE = 1/FEE_RATE_UNIT, so 1e6
pub const FEE_RATE_MUL_VALUE: u128 = 1_000_000;

// Protocol fee rate is represented as a basis point.
// Protocol fee amount = fee_amount * protocol_fee_rate / 10_000.
// Max protocol fee rate supported is 25% of the fee rate.
pub const MAX_PROTOCOL_FEE_RATE: u16 = 2_500;

// Assuming that PROTOCOL_FEE_RATE is represented as a basis point
// We want PROTOCOL_FEE_RATE_MUL_VALUE = 1/PROTOCOL_FEE_UNIT, so 1e4
pub const PROTOCOL_FEE_RATE_MUL_VALUE: u128 = 10_000;
