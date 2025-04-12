use solana_accounts_derive::ToAccountMetas;
use solana_program::pubkey::Pubkey;

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct InitializeConfig {
    #[account(mut, signer = true)]
    pub config: Pubkey,
    #[account(mut, signer = true)]
    pub funder: Pubkey,
    pub system_program: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct InitializePool {
    pub whirlpools_config: Pubkey,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    #[account(mut, signer = true)]
    pub funder: Pubkey,
    #[account(mut)]
    pub whirlpool: Pubkey,
    #[account(mut, signer = true)]
    pub token_vault_a: Pubkey,
    #[account(mut, signer = true)]
    pub token_vault_b: Pubkey,
    pub fee_tier: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct InitializeTickArray {
    pub whirlpool: Pubkey,
    #[account(mut, signer = true)]
    pub funder: Pubkey,
    #[account(mut)]
    pub tick_array: Pubkey,
    pub system_program: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct InitializeFeeTier {
    pub config: Pubkey,

    #[account(mut)]
    pub fee_tier: Pubkey,

    #[account(mut, signer = true)]
    pub funder: Pubkey,

    pub fee_authority: Pubkey,

    pub system_program: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct InitializeReward {
    pub whirlpool: Pubkey,
    #[account(mut, signer = true)]
    pub funder: Pubkey,
    pub reward_mint: Pubkey,
    #[account(mut)]
    pub reward_vault: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct OpenPosition {
    #[account(mut, signer = true)]
    pub funder: Pubkey,

    pub owner: Pubkey,

    #[account(mut)]
    pub position: Pubkey,

    #[account(mut, signer = true)]
    pub position_mint: Pubkey,

    #[account(mut)]
    pub position_token_account: Pubkey,

    pub whirlpool: Pubkey,

    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub associated_token_program: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct ClosePosition {
    #[account(mut, signer = true)]
    pub position_authority: Pubkey,

    /// CHECK: safe, for receiving rent only
    #[account(mut)]
    pub receiver: Pubkey,

    #[account(mut)]
    pub position: Pubkey,

    #[account(mut)]
    pub position_mint: Pubkey,

    #[account(mut)]
    pub position_token_account: Pubkey,

    pub token_program: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct ModifyLiquidity {
    #[account(mut)]
    pub whirlpool: Pubkey,

    pub token_program: Pubkey,

    #[account(mut, signer = true)]
    pub position_authority: Pubkey,

    #[account(mut)]
    pub position: Pubkey,
    pub position_token_account: Pubkey,

    #[account(mut)]
    pub token_owner_account_a: Pubkey,
    #[account(mut)]
    pub token_owner_account_b: Pubkey,

    #[account(mut)]
    pub token_vault_a: Pubkey,
    #[account(mut)]
    pub token_vault_b: Pubkey,

    #[account(mut)]
    pub tick_array_lower: Pubkey,
    #[account(mut)]
    pub tick_array_upper: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct CollectFees {
    pub whirlpool: Pubkey,

    #[account(mut, signer = true)]
    pub position_authority: Pubkey,

    #[account(mut)]
    pub position: Pubkey,
    #[account(mut)]
    pub position_token_account: Pubkey,

    #[account(mut)]
    pub token_owner_account_a: Pubkey,
    #[account(mut)]
    pub token_vault_a: Pubkey,

    #[account(mut)]
    pub token_owner_account_b: Pubkey,
    #[account(mut)]
    pub token_vault_b: Pubkey,

    pub token_program: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct Swap {
    pub token_program: Pubkey,

    #[account(mut, signer = true)]
    pub token_authority: Pubkey,

    #[account(mut)]
    pub whirlpool: Pubkey,

    #[account(mut)]
    pub token_owner_account_a: Pubkey,
    #[account(mut)]
    pub token_vault_a: Pubkey,

    #[account(mut)]
    pub token_owner_account_b: Pubkey,
    #[account(mut)]
    pub token_vault_b: Pubkey,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_0: Pubkey,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_1: Pubkey,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_2: Pubkey,

    pub oracle: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct TwoHopSwap {
    pub token_program: Pubkey,

    #[account(mut, signer = true)]
    pub token_authority: Pubkey,

    #[account(mut)]
    pub whirlpool_one: Pubkey,

    #[account(mut)]
    pub whirlpool_two: Pubkey,

    #[account(mut)]
    pub token_owner_account_one_a: Pubkey,
    #[account(mut)]
    pub token_vault_one_a: Pubkey,

    #[account(mut)]
    pub token_owner_account_one_b: Pubkey,
    #[account(mut)]
    pub token_vault_one_b: Pubkey,

    #[account(mut)]
    pub token_owner_account_two_a: Pubkey,
    #[account(mut)]
    pub token_vault_two_a: Pubkey,

    #[account(mut)]
    pub token_owner_account_two_b: Pubkey,
    #[account(mut)]
    pub token_vault_two_b: Pubkey,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_one_0: Pubkey,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_one_1: Pubkey,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_one_2: Pubkey,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_two_0: Pubkey,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_two_1: Pubkey,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_two_2: Pubkey,

    pub oracle_one: Pubkey,

    pub oracle_two: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct InitializePositionBundle {
    #[account(mut)]
    pub position_bundle: Pubkey,

    #[account(mut)]
    pub position_bundle_mint: Pubkey,

    #[account(mut)]
    pub position_bundle_token_account: Pubkey,

    /// CHECK: safe, the account that will be the owner of the position bundle
    /// can be arbitrary
    pub position_bundle_owner: Pubkey,

    #[account(mut, signer = true)]
    pub funder: Pubkey,

    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
    pub associated_token_program: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct OpenBundledPosition {
    #[account(mut)]
    pub bundled_position: Pubkey,

    #[account(mut)]
    pub position_bundle: Pubkey,

    pub position_bundle_token_account: Pubkey,

    #[account(mut, signer = true)]
    pub position_bundle_authority: Pubkey,

    pub whirlpool: Pubkey,

    #[account(mut, signer = true)]
    pub funder: Pubkey,

    pub system_program: Pubkey,
    pub rent: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct ModifyLiquidityV2 {
    #[account(mut)]
    pub whirlpool: Pubkey,

    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,

    pub memo_program: Pubkey,

    #[account(mut, signer = true)]
    pub position_authority: Pubkey,

    #[account(mut)]
    pub position: Pubkey,
    pub position_token_account: Pubkey,

    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,

    #[account(mut)]
    pub token_owner_account_a: Pubkey,
    #[account(mut)]
    pub token_owner_account_b: Pubkey,

    #[account(mut)]
    pub token_vault_a: Pubkey,
    #[account(mut)]
    pub token_vault_b: Pubkey,

    #[account(mut)]
    pub tick_array_lower: Pubkey,
    #[account(mut)]
    pub tick_array_upper: Pubkey,
    // remaining accounts
    // - accounts for transfer hook program of token_mint_a
    // - accounts for transfer hook program of token_mint_b
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct InitializePoolV2 {
    pub whirlpools_config: Pubkey,

    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,

    pub token_badge_a: Pubkey,
    pub token_badge_b: Pubkey,

    #[account(mut, signer = true)]
    pub funder: Pubkey,

    #[account(mut)]
    pub whirlpool: Pubkey,

    #[account(mut)]
    pub token_vault_a: Pubkey,

    #[account(mut)]
    pub token_vault_b: Pubkey,

    pub fee_tier: Pubkey,

    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct SwapV2 {
    pub token_program_a: Pubkey,
    pub token_program_b: Pubkey,

    pub memo_program: Pubkey,

    #[account(mut, signer = true)]
    pub token_authority: Pubkey,

    #[account(mut)]
    pub whirlpool: Pubkey,

    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,

    #[account(mut)]
    pub token_owner_account_a: Pubkey,
    #[account(mut)]
    pub token_vault_a: Pubkey,

    #[account(mut)]
    pub token_owner_account_b: Pubkey,
    #[account(mut)]
    pub token_vault_b: Pubkey,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_0: Pubkey,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_1: Pubkey,

    #[account(mut)]
    /// CHECK: checked in the handler
    pub tick_array_2: Pubkey,

    #[account(mut)]
    /// CHECK: Oracle is currently unused and will be enabled on subsequent
    /// updates
    pub oracle: Pubkey,
    // remaining accounts
    // - accounts for transfer hook program of token_mint_a
    // - accounts for transfer hook program of token_mint_b
    // - supplemental TickArray accounts
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct TwoHopSwapV2 {
    #[account(mut)]
    pub whirlpool_one: Pubkey,
    #[account(mut)]
    pub whirlpool_two: Pubkey,
    #[account(signer = true)]
    pub token_authority: Pubkey,
    #[account(mut)]
    pub token_owner_account_input: Pubkey,
    #[account(mut)]
    pub token_owner_account_output: Pubkey,
    #[account(mut)]
    pub token_vault_one_input: Pubkey,
    #[account(mut)]
    pub token_vault_one_intermediate: Pubkey,
    #[account(mut)]
    pub token_vault_two_intermediate: Pubkey,
    #[account(mut)]
    pub token_vault_two_output: Pubkey,
    pub token_program: Pubkey,
    #[account(mut)]
    pub tick_array_one_0: Pubkey,
    #[account(mut)]
    pub tick_array_one_1: Pubkey,
    #[account(mut)]
    pub tick_array_one_2: Pubkey,
    #[account(mut)]
    pub tick_array_two_0: Pubkey,
    #[account(mut)]
    pub tick_array_two_1: Pubkey,
    #[account(mut)]
    pub tick_array_two_2: Pubkey,
}
