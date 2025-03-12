use anchor_discriminator_derive::discriminator;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::constants::NUM_REWARDS;

#[derive(BorshSerialize, BorshDeserialize, Clone, Default, Debug, PartialEq)]
pub struct OpenPositionBumps {
    pub position_bump: u8,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Default, Debug, PartialEq)]
pub struct OpenPositionWithMetadataBumps {
    pub position_bump: u8,
    pub metadata_bump: u8,
}

#[discriminator(account)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]

pub struct Position {
    pub whirlpool: Pubkey,     // 32
    pub position_mint: Pubkey, // 32
    pub liquidity: u128,       // 16
    pub tick_lower_index: i32, // 4
    pub tick_upper_index: i32, // 4

    // Q64.64
    pub fee_growth_checkpoint_a: u128, // 16
    pub fee_owed_a: u64,               // 8
    // Q64.64
    pub fee_growth_checkpoint_b: u128, // 16
    pub fee_owed_b: u64,               // 8

    pub reward_infos: [PositionRewardInfo; NUM_REWARDS], // 72
}

#[derive(Copy, Clone, BorshSerialize, BorshDeserialize, Default, Debug, PartialEq)]
pub struct PositionRewardInfo {
    // Q64.64
    pub growth_inside_checkpoint: u128,
    pub amount_owed: u64,
}
