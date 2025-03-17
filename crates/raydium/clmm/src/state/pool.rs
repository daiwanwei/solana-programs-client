use anchor_discriminator_derive::discriminator;
use borsh::{BorshDeserialize, BorshSerialize};
use snafu::{ResultExt, Snafu};
use solana_program::pubkey::Pubkey;

use crate::{
    constants::REWARD_NUM,
    libraries::big_num::U1024,
    math::{tick, tickarray_bitmap},
    state::{RewardInfo, TickArrayBitmapExtension, TickArrayBitmapExtensionError},
};

/// The pool state
///
/// PDA of `[POOL_SEED, config, token_mint_0, token_mint_1]`
#[discriminator(account)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Default, Debug)]
pub struct PoolState {
    /// Bump to identify PDA
    pub bump: [u8; 1],
    // Which config the pool belongs
    pub amm_config: Pubkey,
    // Pool creator
    pub owner: Pubkey,

    /// Token pair of the pool, where token_mint_0 address < token_mint_1
    /// address
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,

    /// Token pair vault
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,

    /// observation account key
    pub observation_key: Pubkey,

    /// mint0 and mint1 decimals
    pub mint_decimals_0: u8,
    pub mint_decimals_1: u8,

    /// The minimum number of ticks between initialized ticks
    pub tick_spacing: u16,
    /// The currently in range liquidity available to the pool.
    pub liquidity: u128,
    /// The current price of the pool as a sqrt(token_1/token_0) Q64.64 value
    pub sqrt_price_x64: u128,
    /// The current tick of the pool, i.e. according to the last tick transition
    /// that was run.
    pub tick_current: i32,

    pub padding3: u16,
    pub padding4: u16,

    /// The fee growth as a Q64.64 number, i.e. fees of token_0 and token_1
    /// collected per unit of liquidity for the entire life of the pool.
    pub fee_growth_global_0_x64: u128,
    pub fee_growth_global_1_x64: u128,

    /// The amounts of token_0 and token_1 that are owed to the protocol.
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,

    /// The amounts in and out of swap token_0 and token_1
    pub swap_in_amount_token_0: u128,
    pub swap_out_amount_token_1: u128,
    pub swap_in_amount_token_1: u128,
    pub swap_out_amount_token_0: u128,

    /// Bitwise representation of the state of the pool
    /// bit0, 1: disable open position and increase liquidity, 0: normal
    /// bit1, 1: disable decrease liquidity, 0: normal
    /// bit2, 1: disable collect fee, 0: normal
    /// bit3, 1: disable collect reward, 0: normal
    /// bit4, 1: disable swap, 0: normal
    pub status: u8,
    /// Leave blank for future use
    pub padding: [u8; 7],

    pub reward_infos: [RewardInfo; REWARD_NUM],

    /// Packed initialized tick array state
    pub tick_array_bitmap: [u64; 16],

    /// except protocol_fee and fund_fee
    pub total_fees_token_0: u64,
    /// except protocol_fee and fund_fee
    pub total_fees_claimed_token_0: u64,
    pub total_fees_token_1: u64,
    pub total_fees_claimed_token_1: u64,

    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,

    // The timestamp allowed for swap in the pool.
    pub open_time: u64,
    // account recent update epoch
    pub recent_epoch: u64,

    // Unused bytes for future upgrades.
    pub padding1: [u64; 24],
    pub padding2: [u64; 32],
}

impl PoolState {
    pub fn next_initialized_tick_array_start_index(
        &self,
        tickarray_bitmap_extension: &Option<TickArrayBitmapExtension>,
        mut last_tick_array_start_index: i32,
        zero_for_one: bool,
    ) -> Result<Option<i32>> {
        last_tick_array_start_index =
            tick::get_array_start_index(last_tick_array_start_index, self.tick_spacing);

        loop {
            let (is_found, start_index) = tickarray_bitmap::next_initialized_tick_array_start_index(
                U1024(self.tick_array_bitmap),
                last_tick_array_start_index,
                self.tick_spacing,
                zero_for_one,
            );
            if is_found {
                return Ok(Some(start_index));
            }
            last_tick_array_start_index = start_index;

            if tickarray_bitmap_extension.is_none() {
                return Err(PoolStateError::MissingTickArrayBitmapExtensionAccount);
            }

            let (is_found, start_index) = tickarray_bitmap_extension
                .as_ref()
                .unwrap()
                .next_initialized_tick_array_from_one_bitmap(
                    last_tick_array_start_index,
                    self.tick_spacing,
                    zero_for_one,
                )
                .context(TickArrayBitmapExtensionSnafu)?;
            if is_found {
                return Ok(Some(start_index));
            }
            last_tick_array_start_index = start_index;

            if last_tick_array_start_index < tick::MIN_TICK
                || last_tick_array_start_index > tick::MAX_TICK
            {
                return Ok(None);
            }
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum PoolStatusBitIndex {
    OpenPositionOrIncreaseLiquidity,
    DecreaseLiquidity,
    CollectFee,
    CollectReward,
    Swap,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum PoolStatusBitFlag {
    Enable,
    Disable,
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum PoolStateError {
    #[snafu(display("Missing tick array bitmap extension account"))]
    MissingTickArrayBitmapExtensionAccount,

    #[snafu(display("Tick array start tick index out of range limit"))]
    TickArrayBitmapExtension { source: TickArrayBitmapExtensionError },
}

pub type Result<T> = std::result::Result<T, PoolStateError>;

#[cfg(test)]
mod tests {
    use anchor_trait::Discriminator;
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    use super::*;

    #[test]
    fn test_deserialize_pool_state() {
        let serialized = "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";

        let decoded = STANDARD.decode(serialized).expect("Failed to decode base64 string");

        let pool_state =
            PoolState::deserialize(&mut &decoded[8..]).expect("Failed to deserialize PoolState");
        println!("pool_state: {:?}", pool_state);
        println!("pool_state discriminator: {:?}", PoolState::DISCRIMINATOR);
        println!("pool_state discriminator 2: {:?}", &decoded[0..8]);
    }
}
