use anchor_trait::Discriminator;
use anchor_trait_derive::discriminator;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::constants::REWARD_NUM;

/// The pool state
///
/// PDA of `[POOL_SEED, config, token_mint_0, token_mint_1]`
#[discriminator(account)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
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

pub enum PoolStatusBitIndex {
    OpenPositionOrIncreaseLiquidity,
    DecreaseLiquidity,
    CollectFee,
    CollectReward,
    Swap,
}

#[derive(PartialEq, Eq)]
pub enum PoolStatusBitFlag {
    Enable,
    Disable,
}

#[derive(Copy, Clone, BorshSerialize, BorshDeserialize, Debug, PartialEq)]
/// State of reward
pub enum RewardState {
    /// Reward not initialized
    Uninitialized,
    /// Reward initialized, but reward time is not start
    Initialized,
    /// Reward in progress
    Opening,
    /// Reward end, reward time expire or
    Ended,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct RewardInfo {
    /// Reward state
    pub reward_state: u8,
    /// Reward open time
    pub open_time: u64,
    /// Reward end time
    pub end_time: u64,
    /// Reward last update time
    pub last_update_time: u64,
    /// Q64.64 number indicates how many tokens per second are earned per unit
    /// of liquidity.
    pub emissions_per_second_x64: u128,
    /// The total amount of reward emissioned
    pub reward_total_emissioned: u64,
    /// The total amount of claimed reward
    pub reward_claimed: u64,
    /// Reward token mint.
    pub token_mint: Pubkey,
    /// Reward vault token account.
    pub token_vault: Pubkey,
    /// The owner that has permission to set reward param
    pub authority: Pubkey,
    /// Q64.64 number that tracks the total tokens earned per unit of liquidity
    /// since the reward emissions were turned on.
    pub reward_growth_global_x64: u128,
}

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    use super::*;

    #[test]
    fn test_deserialize_pool_state() {
        let serialized = "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";

        // 使用 base64 解码
        let decoded = STANDARD.decode(serialized).expect("Failed to decode base64 string");

        // 反序列化为 SwapEvent
        let pool_state =
            PoolState::deserialize(&mut &decoded[8..]).expect("Failed to deserialize PoolState");
        println!("pool_state: {:?}", pool_state);
        println!("pool_state discriminator: {:?}", PoolState::DISCRIMINATOR);
        println!("pool_state discriminator 2: {:?}", &decoded[0..8]);
    }
}
