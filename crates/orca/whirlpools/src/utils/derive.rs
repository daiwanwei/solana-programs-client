use solana_program::pubkey::Pubkey;

use crate::{constants::*, ID};
// 1. Whirlpool PDA
pub fn derive_whirlpool_pubkey(
    whirlpools_config: Pubkey,
    token_mint_a: Pubkey,
    token_mint_b: Pubkey,
    tick_spacing: u16,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    let tick_spacing_bytes = tick_spacing.to_le_bytes();
    Pubkey::find_program_address(
        &[
            WHIRLPOOL_SEED.as_bytes(),
            whirlpools_config.as_ref(),
            token_mint_a.as_ref(),
            token_mint_b.as_ref(),
            tick_spacing_bytes.as_ref(),
        ],
        &program_id,
    )
}

// 2. Position PDA
pub fn derive_position_pubkey(position_mint: Pubkey, program_id: Option<Pubkey>) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(&[POSITION_SEED.as_bytes(), position_mint.as_ref()], &program_id)
}

// 3. Position Bundle PDA
pub fn derive_position_bundle_pubkey(
    position_bundle_mint: Pubkey,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(
        &[POSITION_BUNDLE_SEED.as_bytes(), position_bundle_mint.as_ref()],
        &program_id,
    )
}

// 4. Fee Tier PDA
pub fn derive_fee_tier_pubkey(
    whirlpools_config: Pubkey,
    tick_spacing: u16,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    let tick_spacing_bytes = tick_spacing.to_le_bytes();
    Pubkey::find_program_address(
        &[FEE_TIER_SEED.as_bytes(), whirlpools_config.as_ref(), tick_spacing_bytes.as_ref()],
        &program_id,
    )
}

// 5. Tick Array PDA
pub fn derive_tick_array_pubkey(
    whirlpool: Pubkey,
    start_tick_index: i32,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    let start_tick_index_bytes = start_tick_index.to_le_bytes();
    Pubkey::find_program_address(
        &[TICK_ARRAY_SEED.as_bytes(), whirlpool.as_ref(), start_tick_index_bytes.as_ref()],
        &program_id,
    )
}

// 6. Oracle PDA
pub fn derive_oracle_pubkey(whirlpool: Pubkey, program_id: Option<Pubkey>) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(&[ORACLE_SEED.as_bytes(), whirlpool.as_ref()], &program_id)
}

// 7. Whirlpools Config Extension PDA
pub fn derive_config_extension_pubkey(
    whirlpools_config: Pubkey,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(
        &[CONFIG_EXTENSION_SEED.as_bytes(), whirlpools_config.as_ref()],
        &program_id,
    )
}

// 8. Token Badge PDA
pub fn derive_token_badge_pubkey(
    whirlpools_config: Pubkey,
    token_mint: Pubkey,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(
        &[TOKEN_BADGE_SEED.as_bytes(), whirlpools_config.as_ref(), token_mint.as_ref()],
        &program_id,
    )
}

// 9. Lock Config PDA
pub fn derive_lock_config_pubkey(whirlpool: Pubkey, program_id: Option<Pubkey>) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(&[LOCK_CONFIG_SEED.as_bytes(), whirlpool.as_ref()], &program_id)
}

// 11. Reward PDA
pub fn derive_reward_pubkey(
    whirlpool: Pubkey,
    reward_index: u8,
    program_id: Option<Pubkey>,
) -> (Pubkey, u8) {
    let program_id = program_id.unwrap_or(ID);
    Pubkey::find_program_address(
        &[REWARD_SEED.as_bytes(), whirlpool.as_ref(), &[reward_index]],
        &program_id,
    )
}
