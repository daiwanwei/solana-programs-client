#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]

pub struct DecreaseLiquidityQuote {
    pub liquidity_delta: u128,
    pub token_est_a: u64,
    pub token_est_b: u64,
    pub token_min_a: u64,
    pub token_min_b: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]

pub struct IncreaseLiquidityQuote {
    pub liquidity_delta: u128,
    pub token_est_a: u64,
    pub token_est_b: u64,
    pub token_max_a: u64,
    pub token_max_b: u64,
}
