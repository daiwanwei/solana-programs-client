use crate::constants::TICK_ARRAY_SIZE;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct TickRange {
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct TickFacade {
    pub initialized: bool,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
    pub reward_growths_outside: [u128; 3],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TickArrayFacade {
    pub start_tick_index: i32,
    pub ticks: [TickFacade; TICK_ARRAY_SIZE],
}

impl From<orca_whirlpools::state::TickArray> for TickArrayFacade {
    fn from(tick_array: orca_whirlpools::state::TickArray) -> Self {
        Self {
            start_tick_index: tick_array.start_tick_index,
            ticks: tick_array.ticks.map(|tick| tick.into()),
        }
    }
}

impl From<orca_whirlpools::state::Tick> for TickFacade {
    fn from(tick: orca_whirlpools::state::Tick) -> Self {
        Self {
            initialized: tick.initialized,
            liquidity_net: tick.liquidity_net,
            liquidity_gross: tick.liquidity_gross,
            fee_growth_outside_a: tick.fee_growth_outside_a,
            fee_growth_outside_b: tick.fee_growth_outside_b,
            reward_growths_outside: tick.reward_growths_outside,
        }
    }
}
