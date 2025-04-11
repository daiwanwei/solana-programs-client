use anchor_instructions_derive::Instructions;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Instructions, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhirlpoolInstruction {
    // V1 Instructions
    InitializeConfig {
        fee_authority: Pubkey,
        collect_protocol_fees_authority: Pubkey,
        reward_emissions_super_authority: Pubkey,
        default_protocol_fee_rate: u16,
    },
    InitializePool {
        bumps: WhirlpoolBumps,
        tick_spacing: u16,
        initial_sqrt_price: u128,
    },
    InitializeTickArray {
        start_tick_index: i32,
    },
    InitializeFeeTier {
        tick_spacing: u16,
        default_fee_rate: u16,
    },
    InitializeReward {
        reward_index: u8,
    },
    SetRewardEmissions {
        reward_index: u8,
        emissions_per_second_x64: u128,
    },
    OpenPosition {
        bumps: OpenPositionBumps,
        tick_lower_index: i32,
        tick_upper_index: i32,
    },
    OpenPositionWithMetadata {
        bumps: OpenPositionWithMetadataBumps,
        tick_lower_index: i32,
        tick_upper_index: i32,
    },
    IncreaseLiquidity {
        liquidity_amount: u128,
        token_max_a: u64,
        token_max_b: u64,
    },
    DecreaseLiquidity {
        liquidity_amount: u128,
        token_min_a: u64,
        token_min_b: u64,
    },
    UpdateFeesAndRewards,
    CollectFees,
    CollectReward {
        reward_index: u8,
    },
    CollectProtocolFees,
    Swap {
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool,
    },
    ClosePosition,
    SetDefaultFeeRate {
        default_fee_rate: u16,
    },
    SetDefaultProtocolFeeRate {
        default_protocol_fee_rate: u16,
    },
    SetFeeRate {
        fee_rate: u16,
    },
    SetProtocolFeeRate {
        protocol_fee_rate: u16,
    },
    SetFeeAuthority {
        new_fee_authority: Pubkey,
    },
    SetCollectProtocolFeesAuthority {
        new_collect_protocol_fees_authority: Pubkey,
    },
    SetRewardAuthority {
        reward_index: u8,
        new_reward_authority: Pubkey,
    },
    SetRewardAuthorityBySuperAuthority {
        reward_index: u8,
        new_reward_authority: Pubkey,
    },
    SetRewardEmissionsSuperAuthority {
        new_reward_emissions_super_authority: Pubkey,
    },
    TwoHopSwap {
        amount: u64,
        other_amount_threshold: u64,
        amount_specified_is_input: bool,
        a_to_b_one: bool,
        a_to_b_two: bool,
        sqrt_price_limit_one: u128,
        sqrt_price_limit_two: u128,
    },
    InitializePositionBundle,
    InitializePositionBundleWithMetadata,
    DeletePositionBundle,
    OpenBundledPosition {
        bundle_index: u16,
        tick_lower_index: i32,
        tick_upper_index: i32,
    },
    CloseBundledPosition {
        bundle_index: u16,
    },
    OpenPositionWithTokenExtensions {
        tick_lower_index: i32,
        tick_upper_index: i32,
        with_token_metadata_extension: bool,
    },
    ClosePositionWithTokenExtensions,

    // V2 Instructions
    CollectFeesV2 {
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    CollectProtocolFeesV2 {
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    CollectRewardV2 {
        reward_index: u8,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    DecreaseLiquidityV2 {
        liquidity_amount: u128,
        token_min_a: u64,
        token_min_b: u64,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    IncreaseLiquidityV2 {
        liquidity_amount: u128,
        token_max_a: u64,
        token_max_b: u64,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    InitializePoolV2 {
        tick_spacing: u16,
        initial_sqrt_price: u128,
    },
    InitializeRewardV2 {
        reward_index: u8,
    },
    SetRewardEmissionsV2 {
        reward_index: u8,
        emissions_per_second_x64: u128,
    },
    SwapV2 {
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    TwoHopSwapV2 {
        amount: u64,
        other_amount_threshold: u64,
        amount_specified_is_input: bool,
        a_to_b_one: bool,
        a_to_b_two: bool,
        sqrt_price_limit_one: u128,
        sqrt_price_limit_two: u128,
        remaining_accounts_info: Option<RemainingAccountsInfo>,
    },
    InitializeConfigExtension,
    SetConfigExtensionAuthority {
        new_config_extension_authority: Pubkey,
    },
    SetTokenBadgeAuthority {
        new_token_badge_authority: Pubkey,
    },
    InitializeTokenBadge {
        badge_index: u8,
    },
    DeleteTokenBadge {
        badge_index: u8,
    },
}

// Common Types
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhirlpoolBumps {
    pub whirlpool_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenPositionBumps {
    pub position_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenPositionWithMetadataBumps {
    pub position_bump: u8,
    pub metadata_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemainingAccountsInfo {
    pub remaining_accounts_len: u8,
}
