#![allow(non_snake_case)]

use crate::constants::NUM_REWARDS;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct WhirlpoolFacade {
    pub tick_spacing: u16,
    pub fee_rate: u16,
    pub protocol_fee_rate: u16,
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub tick_current_index: i32,
    pub fee_growth_global_a: u128,
    pub fee_growth_global_b: u128,
    pub reward_last_updated_timestamp: u64,
    pub reward_infos: [WhirlpoolRewardInfoFacade; NUM_REWARDS],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct WhirlpoolRewardInfoFacade {
    pub emissions_per_second_x64: u128,
    pub growth_global_x64: u128,
}

impl From<orca_whirlpools::generated::accounts::Whirlpool> for WhirlpoolFacade {
    fn from(whirlpool: orca_whirlpools::generated::accounts::Whirlpool) -> Self {
        Self {
            tick_spacing: whirlpool.tick_spacing,
            fee_rate: whirlpool.fee_rate,
            protocol_fee_rate: whirlpool.protocol_fee_rate,
            liquidity: whirlpool.liquidity,
            sqrt_price: whirlpool.sqrt_price,
            tick_current_index: whirlpool.tick_current_index,
            fee_growth_global_a: whirlpool.fee_growth_global_a,
            fee_growth_global_b: whirlpool.fee_growth_global_b,
            reward_last_updated_timestamp: whirlpool.reward_last_updated_timestamp,
            reward_infos: whirlpool.reward_infos.map(|reward| reward.into()),
        }
    }
}

impl From<orca_whirlpools::generated::types::WhirlpoolRewardInfo> for WhirlpoolRewardInfoFacade {
    fn from(reward_info: orca_whirlpools::generated::types::WhirlpoolRewardInfo) -> Self {
        Self {
            emissions_per_second_x64: reward_info.emissions_per_second_x64,
            growth_global_x64: reward_info.growth_global_x64,
        }
    }
}
