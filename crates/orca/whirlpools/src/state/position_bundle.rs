use anchor_discriminator_derive::discriminator;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub const POSITION_BITMAP_USIZE: usize = 32;
pub const POSITION_BUNDLE_SIZE: u16 = 8 * POSITION_BITMAP_USIZE as u16;

#[discriminator(account)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct PositionBundle {
    pub position_bundle_mint: Pubkey,                 // 32
    pub position_bitmap: [u8; POSITION_BITMAP_USIZE], // 32
    pub reserved: [u8; 64],                           // 64 RESERVE
}
