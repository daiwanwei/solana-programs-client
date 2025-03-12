use anchor_discriminator_derive::discriminator;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[discriminator(account)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct FeeTier {
    pub whirlpools_config: Pubkey,
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}
