use solana_sdk::pubkey::Pubkey;

pub struct InitializeConfigParams {
    pub owner: Pubkey,
    pub fee_authority: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub reward_emissions_super_authority: Pubkey,
    pub default_protocol_fee_rate: u16,
}

pub struct InitializePoolParams {
    pub whirlpool_creator: Pubkey,
    pub whirlpool_config: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub tick_spacing: u16,
    pub sqrt_price: u128,
}

pub struct OpenPositionParams {
    pub payer: Pubkey,
    pub owner: Pubkey,
    pub whirlpool: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

pub struct IncreaseLiquidityParams {
    pub nft_owner: Pubkey,
    pub whirlpool: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_account_a: Pubkey,
    pub token_account_b: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub liquidity: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
}

pub struct DecreaseLiquidityParams {
    pub nft_owner: Pubkey,
    pub whirlpool: Pubkey,
    pub position_nft_mint: Pubkey,
    pub position: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub recipient_token_account_0: Pubkey,
    pub recipient_token_account_1: Pubkey,
    pub mint_0: Pubkey,
    pub mint_1: Pubkey,
    pub liquidity: u128,
    pub amount_0_min: u64,
    pub amount_1_min: u64,
}

pub struct SwapParams {
    pub token_authority: Pubkey,
    pub whirlpool: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub observation_state: Pubkey,
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
    pub tick_array_0: Pubkey,
    pub tick_array_1: Pubkey,
    pub tick_array_2: Pubkey,
}
