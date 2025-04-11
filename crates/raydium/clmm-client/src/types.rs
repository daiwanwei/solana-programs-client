use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Debug, Default)]
pub struct CreateAmmConfigParams {
    pub owner: Pubkey,
    pub config_index: u16,
    pub tick_spacing: u16,
    pub trade_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub fund_fee_rate: u32,
}

#[derive(Clone, Debug, Default)]
pub struct CreatePoolParams {
    pub sqrt_price_x64: u128,
    pub open_time: u64,
    pub pool_creator: Pubkey,
    pub amm_config: Pubkey,
    pub mint_0: Pubkey,
    pub mint_1: Pubkey,
    pub token_program_0: Pubkey,
    pub token_program_1: Pubkey,
}

#[derive(Clone, Debug, Default)]
pub struct OpenPositionV2Params {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub with_metadata: bool,
    pub base_flag: Option<bool>,
    pub pool_state: Pubkey,
    pub mint_0: Pubkey,
    pub mint_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub payer: Pubkey,
    pub metadata_account: Pubkey,
    pub token_account_0: Pubkey,
    pub token_account_1: Pubkey,
}

#[derive(Clone, Debug, Default)]
pub struct IncreaseLiquidityV2Params {
    pub liquidity: u128,
    pub amount_0_max: u64,
    pub amount_1_max: u64,
    pub pool_state: Pubkey,
    pub position_nft_mint: Pubkey,
    pub token_account_0: Pubkey,
    pub token_account_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub nft_owner: Pubkey,
    pub mint_0: Pubkey,
    pub mint_1: Pubkey,
    pub protocol_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
}

#[derive(Clone, Debug, Default)]
pub struct SwapV2Params {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: u128,
    pub is_base_input: bool,
    pub zero_for_one: bool,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub input_token_account: Pubkey,
    pub output_token_account: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub observation_state: Pubkey,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub tick_array_accounts: Vec<Pubkey>,
    pub payer: Pubkey,
}

#[derive(Clone, Debug, Default)]
pub struct DecreaseLiquidityV2Params {
    pub liquidity: u128,
    pub amount_0_min: u64,
    pub amount_1_min: u64,
    pub pool_state: Pubkey,
    pub position_nft_mint: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub recipient_token_account_0: Pubkey,
    pub recipient_token_account_1: Pubkey,
    pub nft_owner: Pubkey,
    pub mint_0: Pubkey,
    pub mint_1: Pubkey,
    pub protocol_position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
}
