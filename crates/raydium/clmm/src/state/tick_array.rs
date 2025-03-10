use anchor_trait::Discriminator;
use anchor_trait_derive::discriminator;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::constants::{REWARD_NUM, TICK_ARRAY_SIZE_USIZE};

#[discriminator(account)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct TickArrayState {
    pub pool_id: Pubkey,
    pub start_tick_index: i32,
    pub ticks: [TickState; TICK_ARRAY_SIZE_USIZE],
    pub initialized_tick_count: u8,
    // account update recent epoch
    pub recent_epoch: u64,
    // Unused bytes for future upgrades.
    pub padding: [u8; 107],
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct TickState {
    pub tick: i32,
    /// Amount of net liquidity added (subtracted) when tick is crossed from
    /// left to right (right to left)
    pub liquidity_net: i128,
    /// The total position liquidity that references this tick
    pub liquidity_gross: u128,

    /// Fee growth per unit of liquidity on the _other_ side of this tick
    /// (relative to the current tick) only has relative meaning, not
    /// absolute — the value depends on when the tick is initialized
    pub fee_growth_outside_0_x64: u128,
    pub fee_growth_outside_1_x64: u128,

    // Reward growth per unit of liquidity like fee, array of Q64.64
    pub reward_growths_outside_x64: [u128; REWARD_NUM],
    // Unused bytes for future upgrades.
    pub padding: [u32; 13],
}
