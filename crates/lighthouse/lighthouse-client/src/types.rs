pub use lighthouse::types::*;
pub use lighthouse_common::types::*;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

#[derive(Debug, Clone)]
pub struct AccountDeltaInstructions {
    pub memory_id: u8,
    pub account: Pubkey,
    pub before_ix: Vec<Instruction>,
    pub after_ix: Vec<Instruction>,
}
