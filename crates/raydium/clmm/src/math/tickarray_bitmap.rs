/// ! Helper functions to get most and least significant non-zero bits
use thiserror::Error;

use crate::{
    constants::{TICK_ARRAY_BITMAP_SIZE, TICK_ARRAY_SIZE},
    libraries::big_num::{U1024, U512},
    math::tick,
};

pub type TickArryBitmap = [u64; 8];

pub fn max_tick_in_tickarray_bitmap(tick_spacing: u16) -> i32 {
    i32::from(tick_spacing) * TICK_ARRAY_SIZE * TICK_ARRAY_BITMAP_SIZE
}

pub fn get_bitmap_tick_boundary(tick_array_start_index: i32, tick_spacing: u16) -> (i32, i32) {
    let ticks_in_one_bitmap: i32 = max_tick_in_tickarray_bitmap(tick_spacing);
    let mut m = tick_array_start_index.abs() / ticks_in_one_bitmap;
    if tick_array_start_index < 0 && tick_array_start_index.abs() % ticks_in_one_bitmap != 0 {
        m += 1;
    }
    let min_value: i32 = ticks_in_one_bitmap * m;
    if tick_array_start_index < 0 {
        (-min_value, -min_value + ticks_in_one_bitmap)
    } else {
        (min_value, min_value + ticks_in_one_bitmap)
    }
}

pub fn most_significant_bit(x: U1024) -> Option<u16> {
    if x.is_zero() {
        None
    } else {
        Some(u16::try_from(x.leading_zeros()).unwrap())
    }
}

pub fn least_significant_bit(x: U1024) -> Option<u16> {
    if x.is_zero() {
        None
    } else {
        Some(u16::try_from(x.trailing_zeros()).unwrap())
    }
}

/// Given a tick, calculate whether the tickarray it belongs to has been
/// initialized.
pub fn check_current_tick_array_is_initialized(
    bit_map: U1024,
    tick_current: i32,
    tick_spacing: u16,
) -> Result<(bool, i32)> {
    if tick::check_is_out_of_boundary(tick_current) {
        return Err(TickArrayBitmapError::InvalidTickIndex);
    }
    let multiplier = i32::from(tick_spacing) * TICK_ARRAY_SIZE;
    let mut compressed = tick_current / multiplier + 512;
    if tick_current < 0 && tick_current % multiplier != 0 {
        // round towards negative infinity
        compressed -= 1;
    }
    let bit_pos = compressed.abs();
    // set current bit
    let mask = U1024::one() << bit_pos.try_into().unwrap();
    let masked = bit_map & mask;
    // check the current bit whether initialized
    let initialized = masked != U1024::default();
    if initialized {
        return Ok((true, (compressed - 512) * multiplier));
    }
    // the current bit is not initialized
    return Ok((false, (compressed - 512) * multiplier));
}

pub fn next_initialized_tick_array_start_index(
    bit_map: U1024,
    last_tick_array_start_index: i32,
    tick_spacing: u16,
    zero_for_one: bool,
) -> (bool, i32) {
    assert!(tick::check_is_valid_start_index(last_tick_array_start_index, tick_spacing));
    let tick_boundary = max_tick_in_tickarray_bitmap(tick_spacing);
    let next_tick_array_start_index = if zero_for_one {
        last_tick_array_start_index - tick::tick_count(tick_spacing)
    } else {
        last_tick_array_start_index + tick::tick_count(tick_spacing)
    };

    if next_tick_array_start_index < -tick_boundary || next_tick_array_start_index >= tick_boundary
    {
        return (false, last_tick_array_start_index);
    }

    let multiplier = i32::from(tick_spacing) * TICK_ARRAY_SIZE;
    let mut compressed = next_tick_array_start_index / multiplier + 512;
    if next_tick_array_start_index < 0 && next_tick_array_start_index % multiplier != 0 {
        // round towards negative infinity
        compressed -= 1;
    }
    let bit_pos = compressed.abs();

    if zero_for_one {
        // tick from upper to lower
        // find from highter bits to lower bits
        let offset_bit_map = bit_map << (1024 - bit_pos - 1).try_into().unwrap();
        let next_bit = most_significant_bit(offset_bit_map);
        if next_bit.is_some() {
            let next_array_start_index =
                (bit_pos - i32::from(next_bit.unwrap()) - 512) * multiplier;
            (true, next_array_start_index)
        } else {
            // not found til to the end
            (false, -tick_boundary)
        }
    } else {
        // tick from lower to upper
        // find from lower bits to highter bits
        let offset_bit_map = bit_map >> (bit_pos).try_into().unwrap();
        let next_bit = least_significant_bit(offset_bit_map);
        if next_bit.is_some() {
            let next_array_start_index =
                (bit_pos + i32::from(next_bit.unwrap()) - 512) * multiplier;
            (true, next_array_start_index)
        } else {
            // not found til to the end
            (false, tick_boundary - tick::tick_count(tick_spacing))
        }
    }
}

pub fn next_initialized_tick_array_in_bitmap(
    tickarray_bitmap: TickArryBitmap,
    next_tick_array_start_index: i32,
    tick_spacing: u16,
    zero_for_one: bool,
) -> (bool, i32) {
    let (bitmap_min_tick_boundary, bitmap_max_tick_boundary) =
        get_bitmap_tick_boundary(next_tick_array_start_index, tick_spacing);

    let tick_array_offset_in_bitmap =
        tick_array_offset_in_bitmap(next_tick_array_start_index, tick_spacing);
    if zero_for_one {
        // tick from upper to lower
        // find from highter bits to lower bits
        let offset_bit_map =
            U512(tickarray_bitmap) << (TICK_ARRAY_BITMAP_SIZE - 1 - tick_array_offset_in_bitmap);

        let next_bit = if offset_bit_map.is_zero() {
            None
        } else {
            Some(u16::try_from(offset_bit_map.leading_zeros()).unwrap())
        };

        if next_bit.is_some() {
            let next_array_start_index = next_tick_array_start_index
                - i32::from(next_bit.unwrap()) * tick::tick_count(tick_spacing);
            return (true, next_array_start_index);
        } else {
            // not found til to the end
            return (false, bitmap_min_tick_boundary);
        }
    } else {
        // tick from lower to upper
        // find from lower bits to highter bits
        let offset_bit_map = U512(tickarray_bitmap) >> tick_array_offset_in_bitmap;

        let next_bit = if offset_bit_map.is_zero() {
            None
        } else {
            Some(u16::try_from(offset_bit_map.trailing_zeros()).unwrap())
        };
        if next_bit.is_some() {
            let next_array_start_index = next_tick_array_start_index
                + i32::from(next_bit.unwrap()) * tick::tick_count(tick_spacing);
            return (true, next_array_start_index);
        } else {
            // not found til to the end
            return (false, bitmap_max_tick_boundary - tick::tick_count(tick_spacing));
        }
    }
}

pub fn tick_array_offset_in_bitmap(tick_array_start_index: i32, tick_spacing: u16) -> i32 {
    let m = tick_array_start_index.abs() % max_tick_in_tickarray_bitmap(tick_spacing);
    let mut tick_array_offset_in_bitmap = m / tick::tick_count(tick_spacing);
    if tick_array_start_index < 0 && m != 0 {
        tick_array_offset_in_bitmap = TICK_ARRAY_BITMAP_SIZE - tick_array_offset_in_bitmap;
    }
    tick_array_offset_in_bitmap
}

pub fn get_bitmap_offset(tick_index: i32, tick_spacing: u16) -> Result<usize> {
    if tick::check_is_out_of_boundary(tick_index) {
        return Err(TickArrayBitmapError::InvalidTickIndex);
    }
    let ticks_in_one_bitmap = max_tick_in_tickarray_bitmap(tick_spacing);
    let mut offset = tick_index.abs() / ticks_in_one_bitmap - 1;
    if tick_index < 0 && tick_index.abs() % ticks_in_one_bitmap == 0 {
        offset -= 1;
    }
    Ok(offset as usize)
}

#[derive(Debug, Error)]
pub enum TickArrayBitmapError {
    #[error("Invalid tick index")]
    InvalidTickIndex,
}

pub type Result<T> = std::result::Result<T, TickArrayBitmapError>;
