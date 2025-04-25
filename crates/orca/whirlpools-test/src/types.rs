use solana_sdk::{pubkey::Pubkey, signature::Keypair};

#[derive(Clone, Debug)]
pub struct TokenPair {
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub decimals_a: u8,
    pub decimals_b: u8,
    pub token_program_id_a: Pubkey,
    pub token_program_id_b: Pubkey,
}

#[derive(Clone, Debug, Default)]
pub struct InitializeConfigParams {
    pub owner: Pubkey,
    pub fee_authority: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub reward_emissions_super_authority: Pubkey,
    pub default_protocol_fee_rate: u16,
}

#[derive(Clone, Debug, Default)]
pub struct InitializeFeeTierParams {
    pub whirlpool_config: Pubkey,
    pub fee_authority: Pubkey,
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}

#[derive(Clone, Debug, Default)]
pub struct InitializePoolParams {
    pub whirlpool_config: Pubkey,
    pub fee_tier: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub tick_spacing: u16,
    pub sqrt_price: u128,
}

#[derive(Clone, Debug, Default)]
pub struct InitializeTickArraysParams {
    pub whirlpool: Pubkey,
    pub tick_spacing: u16,
    pub start_tick_index: i32,
    pub array_count: u32,
    pub a_to_b: bool,
}

#[derive(Clone, Debug, Default)]
pub struct OpenPositionParams {
    pub owner: Pubkey,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

pub struct IncreaseLiquidityParams {
    pub nft_owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub token_account_a: Pubkey,
    pub token_account_b: Pubkey,
    pub liquidity: u128,
    pub token_max_a: u64,
    pub token_max_b: u64,
}

pub struct DecreaseLiquidityParams {
    pub nft_owner: Pubkey,
    pub position_nft_mint: Pubkey,
    pub tick_array_lower: Pubkey,
    pub tick_array_upper: Pubkey,
    pub recipient_token_account_a: Pubkey,
    pub recipient_token_account_b: Pubkey,
    pub liquidity: u128,
    pub token_min_a: u64,
    pub token_min_b: u64,
}

pub struct SwapParams {
    pub token_authority: Pubkey,
    pub token_owner_account_a: Pubkey,
    pub token_owner_account_b: Pubkey,
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
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
pub struct CreateConfigParams {
    pub fee_authority: Pubkey,
    pub collect_protocol_fees_authority: Pubkey,
    pub reward_emissions_super_authority: Pubkey,
    pub default_protocol_fee_rate: u16,
}

impl Default for CreateConfigParams {
    fn default() -> Self {
        Self {
            fee_authority: Pubkey::default(),
            collect_protocol_fees_authority: Pubkey::default(),
            reward_emissions_super_authority: Pubkey::default(),
            default_protocol_fee_rate: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateFeeTierParams {
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}

impl Default for CreateFeeTierParams {
    fn default() -> Self { Self { tick_spacing: 1, default_fee_rate: 0 } }
}

#[derive(Clone, Debug, Default)]
pub struct CreatePoolParams {
    pub sqrt_price: u128,
}

#[derive(Clone, Debug)]
pub struct CreateWhirlPoolTesterParams {
    pub program_id: Pubkey,
    pub whirlpool_config: Pubkey,
    pub fee_tier: Pubkey,
    pub whirlpool: Pubkey,
    pub token_pair: TokenPair,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_spacing: u16,
}

#[derive(Clone, Debug)]
pub struct WhirlpoolConfigFixture {
    pub whirlpool_config: Pubkey,
    pub fee_tier_list: Vec<FeeTierInfo>,
}

#[derive(Clone, Debug, Default)]
pub struct FeeTierInfo {
    pub fee_tier: Pubkey,
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}

#[derive(Clone, Debug)]
pub struct WhirlpoolFixture {
    pub program_id: Pubkey,
    pub whirlpool_config: Pubkey,
    pub fee_tier: Pubkey,
    pub whirlpool: Pubkey,
    pub token_pair: TokenPair,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_spacing: u16,
}

#[derive(Clone, Debug, Default)]
pub struct SetupTickArraysParams {
    pub whirlpool: Pubkey,
    pub start_tick: i32,
    pub tick_spacing: u16,
    pub a_to_b: bool,
    pub array_count: u32,
}

#[derive(Debug)]
pub struct User {
    pub keypair: Keypair,
    pub token_account_0: Pubkey,
    pub token_account_1: Pubkey,
}
