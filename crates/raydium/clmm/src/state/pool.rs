use borsh::{BorshDeserialize, BorshSerialize};
use snafu::Snafu;

use crate::{
    generated::accounts::{PoolState, TickArrayBitmapExtension},
    libraries::big_num::U1024,
    math::{tick, tickarray_bitmap},
};

impl PoolState {
    pub fn next_initialized_tick_array_start_index(
        &self,
        tickarray_bitmap_extension: &Option<TickArrayBitmapExtension>,
        mut last_tick_array_start_index: i32,
        zero_for_one: bool,
    ) -> PoolResult<Option<i32>> {
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
                .map_err(|_| PoolStateError::TickArrayBitmapExtension)?;
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

    pub fn get_first_initialized_tick_array(
        &self,
        tickarray_bitmap_extension: &Option<TickArrayBitmapExtension>,
        zero_for_one: bool,
    ) -> PoolResult<(bool, i32)> {
        let (is_initialized, start_index) =
            if self.is_overflow_default_tickarray_bitmap(vec![self.tick_current]) {
                tickarray_bitmap_extension
                    .as_ref()
                    .unwrap()
                    .check_tick_array_is_initialized(
                        tick::get_array_start_index(self.tick_current, self.tick_spacing),
                        self.tick_spacing,
                    )
                    .unwrap()
            } else {
                tickarray_bitmap::check_current_tick_array_is_initialized(
                    U1024(self.tick_array_bitmap),
                    self.tick_current,
                    self.tick_spacing.into(),
                )
                .unwrap()
            };
        if is_initialized {
            return Ok((true, start_index));
        }
        let next_start_index = self
            .next_initialized_tick_array_start_index(
                tickarray_bitmap_extension,
                tick::get_array_start_index(self.tick_current, self.tick_spacing),
                zero_for_one,
            )
            .unwrap();
        // require!(
        //     next_start_index.is_some(),
        //     PoolStateError::InsufficientLiquidityForDirection
        // );
        return Ok((false, next_start_index.unwrap()));
    }

    pub fn is_overflow_default_tickarray_bitmap(&self, tick_indexs: Vec<i32>) -> bool {
        let (min_tick_array_start_index_boundary, max_tick_array_index_boundary) =
            self.tick_array_start_index_range();
        for tick_index in tick_indexs {
            let tick_array_start_index = tick::get_array_start_index(tick_index, self.tick_spacing);
            if tick_array_start_index >= max_tick_array_index_boundary
                || tick_array_start_index < min_tick_array_start_index_boundary
            {
                return true;
            }
        }
        false
    }

    // the range of tick array start index that default tickarray bitmap can
    // represent if tick_spacing = 1, the result range is [-30720, 30720)
    pub fn tick_array_start_index_range(&self) -> (i32, i32) {
        // the range of ticks that default tickarrary can represent
        let mut max_tick_boundary =
            tickarray_bitmap::max_tick_in_tickarray_bitmap(self.tick_spacing);
        let mut min_tick_boundary = -max_tick_boundary;
        if max_tick_boundary > tick::MAX_TICK {
            max_tick_boundary = tick::get_array_start_index(tick::MAX_TICK, self.tick_spacing);
            // find the next tick array start index
            max_tick_boundary = max_tick_boundary + tick::tick_count(self.tick_spacing);
        }
        if min_tick_boundary < tick::MIN_TICK {
            min_tick_boundary = tick::get_array_start_index(tick::MIN_TICK, self.tick_spacing);
        }
        (min_tick_boundary, max_tick_boundary)
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
    TickArrayBitmapExtension,
}

pub type PoolResult<T> = Result<T, PoolStateError>;
