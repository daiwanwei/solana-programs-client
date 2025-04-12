use litesvm::LiteSVM;
use program_test_utils::account::{get_anchor_account, get_anchor_accounts, get_solana_account};
use solana_sdk::pubkey::Pubkey;

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
        svm: &LiteSVM,
        whirlpool: &Pubkey,
    ) -> Result<orca_whirlpools::state::Whirlpool> {
        let whirlpool_account =
            get_anchor_account::<orca_whirlpools::state::Whirlpool>(svm, whirlpool)
                .ok_or(WhirlpoolsTestError::WhirlpoolNotFound)?;

        Ok(whirlpool_account.data)
    }

    pub fn get_position(
        &self,
        svm: &LiteSVM,
        position: &Pubkey,
    ) -> Result<orca_whirlpools::state::Position> {
        let position_account =
            get_anchor_account::<orca_whirlpools::state::Position>(svm, position)
                .ok_or(WhirlpoolsTestError::PositionNotFound)?;

        Ok(position_account.data)
    }

    pub fn get_tick_arrays(
        &self,
        svm: &LiteSVM,
        tick_arrays: &[Pubkey],
    ) -> Result<Vec<orca_whirlpools::state::TickArray>> {
        let tick_arrays =
            get_anchor_accounts::<orca_whirlpools::state::TickArray>(svm, tick_arrays)
                .into_iter()
                .filter_map(
                    |account| {
                        if let Some(account) = account {
                            Some(account.data)
                        } else {
                            None
                        }
                    },
                )
                .collect();

        Ok(tick_arrays)
    }

    pub fn get_token_account(
        &self,
        svm: &LiteSVM,
        token_account: &Pubkey,
    ) -> Result<spl_token::state::Account> {
        let token_account_account =
            get_solana_account::<spl_token::state::Account>(svm, token_account)
                .ok_or(WhirlpoolsTestError::TokenAccountNotFound)?;

        Ok(token_account_account.data)
    }
}
