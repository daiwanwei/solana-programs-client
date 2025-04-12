use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Debug)]
pub struct TokenPair {
    pub mint_0: Pubkey,
    pub mint_1: Pubkey,
    pub decimals_0: u8,
    pub decimals_1: u8,
    pub token_program_id_0: Pubkey,
    pub token_program_id_1: Pubkey,
}

#[derive(Clone, Debug)]
pub struct FeeConfig {
    pub tick_spacing: u16,
    pub trade_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub fund_fee_rate: u32,
}

impl Default for FeeConfig {
    fn default() -> Self {
        Self { tick_spacing: 1, trade_fee_rate: 0, protocol_fee_rate: 0, fund_fee_rate: 0 }
    }
}

#[derive(Clone, Debug)]
pub struct CreateMintsParams {
    pub decimals_a: u8,
    pub decimals_b: u8,
    pub token_program_id_a: Option<Pubkey>,
    pub token_program_id_b: Option<Pubkey>,
}

impl Default for CreateMintsParams {
    fn default() -> Self {
        Self {
            decimals_a: 6,
            decimals_b: 6,
            token_program_id_a: Some(spl_token::ID),
            token_program_id_b: Some(spl_token::ID),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateAmmConfigParams {
    pub config_index: u16,
    pub tick_spacing: u16,
    pub trade_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub fund_fee_rate: u32,
}

impl Default for CreateAmmConfigParams {
    fn default() -> Self {
        Self {
            config_index: 0,
            tick_spacing: 1,
            trade_fee_rate: 0,
            protocol_fee_rate: 0,
            fund_fee_rate: 0,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CreatePoolParams {
    pub sqrt_price_x64: u128,
    pub open_time: u64,
}

#[derive(Clone, Debug, Default)]
pub struct OpenPositionV2Params {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub user_token_account_0: Pubkey,
    pub user_token_account_1: Pubkey,
}

#[derive(Clone, Debug, Default)]
pub struct IncreaseLiquidityV2Params {
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub position_nft_mint: Pubkey,
    pub user_token_account_0: Pubkey,
    pub user_token_account_1: Pubkey,
}

#[derive(Clone, Debug, Default)]
pub struct SwapV2Params {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
    pub zero_for_one: bool,
    pub user_token_account_0: Pubkey,
    pub user_token_account_1: Pubkey,
}

#[derive(Clone, Debug, Default)]
pub struct DecreaseLiquidityV2Params {
    pub liquidity: u128,
    pub amount_0_min: u64,
    pub amount_1_min: u64,
    pub position_nft_mint: Pubkey,
    pub recipient_token_account_0: Pubkey,
    pub recipient_token_account_1: Pubkey,
}
