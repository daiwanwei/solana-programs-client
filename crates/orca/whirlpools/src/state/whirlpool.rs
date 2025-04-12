use anchor_discriminator_derive::discriminator;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::constants::NUM_REWARDS;

#[discriminator(account)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Default, Debug)]
pub struct Whirlpool {
    pub whirlpools_config: Pubkey, // 32
    pub whirlpool_bump: [u8; 1],   // 1

    pub tick_spacing: u16,          // 2
    pub tick_spacing_seed: [u8; 2], // 2

    // Stored as hundredths of a basis point
    // u16::MAX corresponds to ~6.5%
    pub fee_rate: u16, // 2

    // Portion of fee rate taken stored as basis points
    pub protocol_fee_rate: u16, // 2

    // Maximum amount that can be held by Solana account
    pub liquidity: u128, // 16

    // MAX/MIN at Q32.64, but using Q64.64 for rounder bytes
    // Q64.64
    pub sqrt_price: u128,        // 16
    pub tick_current_index: i32, // 4

    pub protocol_fee_owed_a: u64, // 8
    pub protocol_fee_owed_b: u64, // 8

    pub token_mint_a: Pubkey,  // 32
    pub token_vault_a: Pubkey, // 32

    // Q64.64
    pub fee_growth_global_a: u128, // 16

    pub token_mint_b: Pubkey,  // 32
    pub token_vault_b: Pubkey, // 32

    // Q64.64
    pub fee_growth_global_b: u128, // 16

    pub reward_last_updated_timestamp: u64, // 8

    pub reward_infos: [WhirlpoolRewardInfo; NUM_REWARDS], // 384
}

/// Stores the state relevant for tracking liquidity mining rewards at the
/// `Whirlpool` level. These values are used in conjunction with
/// `PositionRewardInfo`, `Tick.reward_growths_outside`, and `Whirlpool.
/// reward_last_updated_timestamp` to determine how many rewards are earned by
/// open positions.
#[derive(Copy, Clone, BorshSerialize, BorshDeserialize, Default, Debug, PartialEq)]
pub struct WhirlpoolRewardInfo {
    /// Reward token mint.
    pub mint: Pubkey,
    /// Reward vault token account.
    pub vault: Pubkey,
    /// Authority account that has permission to initialize the reward and set
    /// emissions.
    pub authority: Pubkey,
    /// Q64.64 number that indicates how many tokens per second are earned per
    /// unit of liquidity.
    pub emissions_per_second_x64: u128,
    /// Q64.64 number that tracks the total tokens earned per unit of liquidity
    /// since the reward emissions were turned on.
    pub growth_global_x64: u128,
}

impl WhirlpoolRewardInfo {
    /// Creates a new `WhirlpoolRewardInfo` with the authority set
    pub fn new(authority: Pubkey) -> Self { Self { authority, ..Default::default() } }

    /// Returns true if this reward is initialized.
    /// Once initialized, a reward cannot transition back to uninitialized.
    pub fn initialized(&self) -> bool { self.mint.ne(&Pubkey::default()) }

    /// Maps all reward data to only the reward growth accumulators
    pub fn to_reward_growths(
        reward_infos: &[WhirlpoolRewardInfo; NUM_REWARDS],
    ) -> [u128; NUM_REWARDS] {
        let mut reward_growths = [0u128; NUM_REWARDS];
        for i in 0..NUM_REWARDS {
            reward_growths[i] = reward_infos[i].growth_global_x64;
        }
        reward_growths
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Default, Copy)]
pub struct WhirlpoolBumps {
    pub whirlpool_bump: u8,
}

#[cfg(test)]
mod tests {
    use anchor_trait::Discriminator;
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    use super::*;

    #[test]
    fn test_deserialize_whirlpool() {
        let serialized = "P5XRDOGAYwkT5EH4ORPKaLBjT7Al/eqohzfoQRDRJV41ezN33e4czf8EAAQAkAEUBTAOFLLvMgEAAAAAAAAAAABbndwysOC8WgAAAAAAAAAA9q7//0ilihIAAAAAVGHYAwAAAAAGm4hX/quBhPtof2NGGMA12sQ53BrrO1WYoPAAAAAAAchN8kM4mDvkqFswl7r0C8lXEQjSiawAs2jfF11Edc96wJk9eW3kWXwAAAAAAAAAAMb6evO+2606PWXzaqvJdDGxu+TC0vbg5HymAgNFL11hFl+VcsWpaqUC3VEQVKJqbSWO98HW1sGu4SkZFNxRAjKhyj3mtUcBEAAAAAAAAAAA3W7RZwAAAAAMANCv64YU2n8Zq6AtQPGMaSWF9lAg387T1eX5qcDE4Q8bkJQIzrVDfhKReyB9qZTQ6FenQB4SLAPfa/fG1/wqvR0xrxfe/zwmhIFgCsr+SxQJjA/hQbf0oc34STRkRAMAAAAAAAAAAAAAAAAAAAAAIxHh3tFPDkQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAC9HTGvF97/PCaEgWAKyv5LFAmMD+FBt/ShzfhJNGREAwAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAL0dMa8X3v88JoSBYArK/ksUCYwP4UG39KHN+Ek0ZEQDAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";

        let decoded = STANDARD.decode(serialized).expect("Failed to decode base64 string");

        let whirlpool =
            Whirlpool::deserialize(&mut &decoded[8..]).expect("Failed to deserialize PoolState");
        println!("whirlpool state: {:?}", whirlpool);
        println!("whirlpool discriminator: {:?}", Whirlpool::DISCRIMINATOR);
        println!("whirlpool discriminator 2: {:?}", &decoded[0..8]);
    }
}
