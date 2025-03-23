use anchor_discriminator_derive::discriminator;
use borsh::{BorshDeserialize, BorshSerialize};
use snafu::{ResultExt, Snafu};
use solana_program::pubkey::Pubkey;

use crate::{
    constants::EXTENSION_TICKARRAY_BITMAP_SIZE,
    math::{tick, tickarray_bitmap},
};

#[discriminator(account)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct TickArrayBitmapExtension {
    pub pool_id: Pubkey,
    /// Packed initialized tick array state for start_tick_index is positive
    pub positive_tick_array_bitmap: [[u64; 8]; EXTENSION_TICKARRAY_BITMAP_SIZE],
    /// Packed initialized tick array state for start_tick_index is negitive
    pub negative_tick_array_bitmap: [[u64; 8]; EXTENSION_TICKARRAY_BITMAP_SIZE],
}

impl Default for TickArrayBitmapExtension {
    #[inline]
    fn default() -> TickArrayBitmapExtension {
        TickArrayBitmapExtension {
            pool_id: Pubkey::default(),
            positive_tick_array_bitmap: [[0; 8]; EXTENSION_TICKARRAY_BITMAP_SIZE],
            negative_tick_array_bitmap: [[0; 8]; EXTENSION_TICKARRAY_BITMAP_SIZE],
        }
    }
}

impl TickArrayBitmapExtension {
    /// According to the given tick, calculate its corresponding tickarray and
    /// then find the bitmap it belongs to.
    fn get_bitmap(
        &self,
        tick_index: i32,
        tick_spacing: u16,
    ) -> TickArrayBitmapResult<(usize, tickarray_bitmap::TickArryBitmap)> {
        let offset = tickarray_bitmap::get_bitmap_offset(tick_index, tick_spacing)
            .context(TickArrayBitmapSnafu)?;
        if tick_index < 0 {
            Ok((offset, self.negative_tick_array_bitmap[offset]))
        } else {
            Ok((offset, self.positive_tick_array_bitmap[offset]))
        }
    }

    /// Search for the first initialized bit in bitmap according to the
    /// direction, if found return ture and the tick array start index,
    /// if not, return false and tick boundary index
    pub fn next_initialized_tick_array_from_one_bitmap(
        &self,
        last_tick_array_start_index: i32,
        tick_spacing: u16,
        zero_for_one: bool,
    ) -> TickArrayBitmapResult<(bool, i32)> {
        let multiplier = tick::tick_count(tick_spacing);
        let next_tick_array_start_index = if zero_for_one {
            last_tick_array_start_index - multiplier
        } else {
            last_tick_array_start_index + multiplier
        };
        let min_tick_array_start_index = tick::get_array_start_index(tick::MIN_TICK, tick_spacing);
        let max_tick_array_start_index = tick::get_array_start_index(tick::MAX_TICK, tick_spacing);

        if next_tick_array_start_index < min_tick_array_start_index
            || next_tick_array_start_index > max_tick_array_start_index
        {
            return Ok((false, next_tick_array_start_index));
        }

        let (_, tickarray_bitmap) = self.get_bitmap(next_tick_array_start_index, tick_spacing)?;

        Ok(tickarray_bitmap::next_initialized_tick_array_in_bitmap(
            tickarray_bitmap,
            next_tick_array_start_index,
            tick_spacing,
            zero_for_one,
        ))
    }
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum TickArrayBitmapExtensionError {
    #[snafu(display("Invalid tick index"))]
    InvalidTickIndex,

    #[snafu(display("Tick array bitmap error: {}", source))]
    TickArrayBitmap { source: tickarray_bitmap::TickArrayBitmapError },
}

pub type TickArrayBitmapResult<T> = Result<T, TickArrayBitmapExtensionError>;
