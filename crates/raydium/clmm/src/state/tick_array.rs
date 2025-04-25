use snafu::Snafu;

use crate::{
    constants::{TICK_ARRAY_SIZE, TICK_ARRAY_SIZE_USIZE},
    generated::{accounts::TickArrayState, types::TickState},
    math::tick,
};

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
            // Note: different from the original clmm, we need to check if the
            // offset_in_array is 0
            if offset_in_array == 0 {
                return Ok(None);
            }

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
