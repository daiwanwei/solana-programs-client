use anchor_discriminator_derive::discriminator;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[discriminator(account)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct TokenBadge {
    pub whirlpools_config: Pubkey, // 32
    pub token_mint: Pubkey,        // 32
    pub reserved: [u8; 128],       // 128
}
