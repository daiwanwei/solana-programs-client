use litesvm::LiteSVM;
use program_test_utils::account::get_anchor_account;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
};
use spl_token::ID as SPL_TOKEN_ID;

use crate::{
    builder::WhirlpoolsTestBuilder,
    error::{Result, WhirlpoolsTestError},
    types::*,
};

pub struct WhirlpoolsTest {
    pub program_id: Pubkey,
    pub token_pair: TokenPair,
    pub whirlpool_config: Pubkey,
    pub whirlpool: Pubkey,
    pub fee_tier: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_spacing: u16,
}

impl WhirlpoolsTest {
    pub fn builder() -> WhirlpoolsTestBuilder { WhirlpoolsTestBuilder::new() }

    pub fn get_whirlpool(
        &self,
        svm: &mut LiteSVM,
        whirlpool: &Pubkey,
    ) -> Result<orca_whirlpools::state::Whirlpool> {
        let whirlpool_account =
            get_anchor_account::<orca_whirlpools::state::Whirlpool>(svm, whirlpool)
                .ok_or(WhirlpoolsTestError::WhirlpoolNotFound)?;

        Ok(whirlpool_account.data)
    }

    pub fn get_position(
        &self,
        svm: &mut LiteSVM,
        position: &Pubkey,
    ) -> Result<orca_whirlpools::state::Position> {
        let position_account =
            get_anchor_account::<orca_whirlpools::state::Position>(svm, position)
                .ok_or(WhirlpoolsTestError::PositionNotFound)?;

        Ok(position_account.data)
    }
}
