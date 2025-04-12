use crate::constants::NUM_REWARDS;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct PositionRatio {
    pub ratio_a: u16,
    pub ratio_b: u16,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PositionStatus {
    PriceInRange,
    PriceBelowRange,
    PriceAboveRange,
    Invalid,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct PositionFacade {
    pub liquidity: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub fee_growth_checkpoint_a: u128,
    pub fee_owed_a: u64,
    pub fee_growth_checkpoint_b: u128,
    pub fee_owed_b: u64,
    pub reward_infos: [PositionRewardInfoFacade; NUM_REWARDS],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct PositionRewardInfoFacade {
    pub growth_inside_checkpoint: u128,
    pub amount_owed: u64,
}
