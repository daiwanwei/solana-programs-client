use ethnum::U256;

use crate::constants::POSITION_BUNDLE_SIZE;

const POSITION_BUNDLE_BYTES: usize = POSITION_BUNDLE_SIZE / 8;

/// Get the first unoccupied position in a bundle
///
/// # Arguments
/// * `bundle` - The bundle to check
///
/// # Returns
/// * `u32` - The first unoccupied position (None if full)
pub fn first_unoccupied_position_in_bundle(bitmap: &[u8]) -> Option<u32> {
    let value = bitmap_to_u256(bitmap);
    for i in 0..POSITION_BUNDLE_SIZE {
        if value & (U256::ONE << i) == 0 {
            return Some(i as u32);
        }
    }
    None
}

/// Check whether a position bundle is full
/// A position bundle can contain 256 positions
///
/// # Arguments
/// * `bundle` - The bundle to check
///
/// # Returns
/// * `bool` - Whether the bundle is full
pub fn is_position_bundle_full(bitmap: &[u8]) -> bool {
    let value = bitmap_to_u256(bitmap);
    value == U256::MAX
}

/// Check whether a position bundle is empty
///
/// # Arguments
/// * `bundle` - The bundle to check
///
/// # Returns
/// * `bool` - Whether the bundle is empty
pub fn is_position_bundle_empty(bitmap: &[u8]) -> bool {
    let value = bitmap_to_u256(bitmap);
    value == U256::MIN
}

// Private functions

#[allow(clippy::needless_range_loop)]
fn bitmap_to_u256(bitmap: &[u8]) -> U256 {
    let mut u256 = <U256>::from(0u32);
    for i in 0..POSITION_BUNDLE_BYTES {
        let byte = bitmap[i];
        u256 += <U256>::from(byte) << (i * 8);
    }
    u256
}
