use crate::constants::TICK_ARRAY_SIZE;

/// Get tick's offset in current tick array, tick must be include in tick
/// array， otherwise throw an error
pub fn get_tick_offset_in_array(
    start_tick_index: i32,
    tick_index: i32,
    tick_spacing: u16,
) -> usize {
    let offset_in_array = ((tick_index - start_tick_index) / i32::from(tick_spacing)) as usize;
    offset_in_array
}

/// Input an arbitrary tick_index, output the start_index of the tick_array it
/// sits on
pub fn get_array_start_index(tick_index: i32, tick_spacing: u16) -> i32 {
    let ticks_in_array = tick_count(tick_spacing);
    let mut start = tick_index / ticks_in_array;
    if tick_index < 0 && tick_index % ticks_in_array != 0 {
        start = start - 1
    }
    start * ticks_in_array
}

pub fn tick_count(tick_spacing: u16) -> i32 { TICK_ARRAY_SIZE * i32::from(tick_spacing) }
