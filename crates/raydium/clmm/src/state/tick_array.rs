use anchor_discriminator_derive::discriminator;
use borsh::{BorshDeserialize, BorshSerialize};
use snafu::Snafu;
use solana_program::pubkey::Pubkey;

use crate::{
    constants::{REWARD_NUM, TICK_ARRAY_SIZE, TICK_ARRAY_SIZE_USIZE},
    math::tick,
};

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

impl TickArrayState {
    /// Get next initialized tick in tick array, `current_tick_index` can be any
    /// tick index, in other words, `current_tick_index` not exactly a point in
    /// the tickarray, and current_tick_index % tick_spacing maybe not equal
    /// zero. If price move to left tick <= current_tick_index, or to right
    /// tick > current_tick_index
    pub fn next_initialized_tick(
        &self,
        current_tick_index: i32,
        tick_spacing: u16,
        zero_for_one: bool,
    ) -> Result<Option<TickState>> {
        let current_tick_array_start_index =
            tick::get_array_start_index(current_tick_index, tick_spacing);
        if current_tick_array_start_index != self.start_tick_index {
            return Ok(None);
        }
        let mut offset_in_array =
            (current_tick_index - self.start_tick_index) / i32::from(tick_spacing);

        if zero_for_one {
            while offset_in_array >= 0 {
                if self.ticks[offset_in_array as usize].is_initialized() {
                    return Ok(Some(self.ticks[offset_in_array as usize].clone()));
                }
                offset_in_array = offset_in_array - 1;
            }
        } else {
            offset_in_array = offset_in_array + 1;
            while offset_in_array < TICK_ARRAY_SIZE {
                if self.ticks[offset_in_array as usize].is_initialized() {
                    return Ok(Some(self.ticks[offset_in_array as usize].clone()));
                }
                offset_in_array = offset_in_array + 1;
            }
        }
        Ok(None)
    }

    /// Base on swap directioin, return the first initialized tick in the tick
    /// array.
    pub fn first_initialized_tick(&self, zero_for_one: bool) -> Result<TickState> {
        if zero_for_one {
            let mut i = TICK_ARRAY_SIZE - 1;
            while i >= 0 {
                if self.ticks[i as usize].is_initialized() {
                    return Ok(self.ticks[i as usize].clone());
                }
                i = i - 1;
            }
        } else {
            let mut i = 0;
            while i < TICK_ARRAY_SIZE_USIZE {
                if self.ticks[i].is_initialized() {
                    return Ok(self.ticks[i].clone());
                }
                i = i + 1;
            }
        }

        return Err(TickStateError::TickStateNotInitialized);
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default)]
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

impl TickState {
    pub fn is_initialized(&self) -> bool { self.liquidity_gross != 0 }
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum TickStateError {
    #[snafu(display("Tick state not initialized"))]
    TickStateNotInitialized,
}

pub type Result<T> = std::result::Result<T, TickStateError>;
