#![allow(non_snake_case)]

use crate::constants::NUM_REWARDS;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct CollectRewardsQuote {
    pub rewards: [CollectRewardQuote; NUM_REWARDS],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct CollectRewardQuote {
    pub rewards_owed: u64,
}
