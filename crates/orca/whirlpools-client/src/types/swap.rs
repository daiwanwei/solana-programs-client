#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]

pub struct ExactInSwapQuote {
    pub token_in: u64,
    pub token_est_out: u64,
    pub token_min_out: u64,
    pub trade_fee: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]

pub struct ExactOutSwapQuote {
    pub token_out: u64,
    pub token_est_in: u64,
    pub token_max_in: u64,
    pub trade_fee: u64,
}
