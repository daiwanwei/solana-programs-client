use litesvm::LiteSVM;
use program_test_utils::account::{get_solana_account_by_borsh, get_solana_account_by_pack};
use raydium_clmm::{generated, utils::derive};
use rust_decimal::Decimal;
use solana_client_core::types::MaybeAccount;
use solana_sdk::pubkey::Pubkey;

use crate::{
    error::{ClmmTestError, Result},
    types::TokenPair,
};

pub struct RaydiumClmmTest {
    pub program_id: Pubkey,
    pub token_pair: TokenPair,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub observation_state: Pubkey,
    pub tick_array_bitmap: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub fee_config: crate::types::FeeConfig,
}

impl RaydiumClmmTest {
    pub fn get_amm_config(&self, svm: &LiteSVM) -> Result<generated::accounts::AmmConfig> {
        let amm_config = if let MaybeAccount::Exists(account) =
            get_solana_account_by_borsh::<generated::accounts::AmmConfig>(svm, &self.amm_config)
        {
            account
        } else {
            return Err(ClmmTestError::AmmConfigNotFound.into());
        };

        Ok(amm_config.data)
    }

    pub fn get_pool_state(&self, svm: &LiteSVM) -> Result<generated::accounts::PoolState> {
        let pool_state = if let MaybeAccount::Exists(account) =
            get_solana_account_by_borsh::<generated::accounts::PoolState>(svm, &self.pool_state)
        {
            account
        } else {
            return Err(ClmmTestError::PoolStateNotFound.into());
        };

        Ok(pool_state.data)
    }

    pub fn get_tick_array_bitmap(
        &self,
        svm: &LiteSVM,
    ) -> Result<generated::accounts::TickArrayBitmapExtension> {
        let tick_array_bitmap = if let MaybeAccount::Exists(account) =
            get_solana_account_by_borsh::<generated::accounts::TickArrayBitmapExtension>(
                svm,
                &self.tick_array_bitmap,
            ) {
            account
        } else {
            return Err(ClmmTestError::TickArrayBitmapNotFound.into());
        };

        Ok(tick_array_bitmap.data)
    }

    pub fn get_personal_position_state(
        &self,
        svm: &LiteSVM,
        position_nft_mint: Pubkey,
    ) -> Result<generated::accounts::PersonalPositionState> {
        let personal_position =
            derive::derive_personal_position_pubkey(position_nft_mint, Some(self.program_id)).0;
        let personal_position = if let MaybeAccount::Exists(account) = get_solana_account_by_borsh::<
            generated::accounts::PersonalPositionState,
        >(
            &svm, &personal_position
        ) {
            account
        } else {
            return Err(ClmmTestError::PersonalPositionNotFound.into());
        };

        Ok(personal_position.data)
    }

    pub fn get_price(&self, svm: &LiteSVM) -> Result<Decimal> {
        let pool_state = self.get_pool_state(svm)?;
        Ok(raydium_clmm_client::math::price::calculate_price(
            pool_state.sqrt_price_x64,
            self.token_pair.decimals_0,
            self.token_pair.decimals_1,
        ))
    }

    pub fn get_tick_array(
        &self,
        svm: &LiteSVM,
        tick_array_account: Pubkey,
    ) -> Result<generated::accounts::TickArrayState> {
        let tick_array = if let MaybeAccount::Exists(account) = get_solana_account_by_borsh::<
            generated::accounts::TickArrayState,
        >(svm, &tick_array_account)
        {
            account
        } else {
            return Err(ClmmTestError::TickArrayNotFound.into());
        };

        Ok(tick_array.data)
    }

    pub fn get_token_account(
        &self,
        svm: &LiteSVM,
        token_account: Pubkey,
    ) -> Result<spl_token::state::Account> {
        let token_account = if let MaybeAccount::Exists(account) =
            get_solana_account_by_pack::<spl_token::state::Account>(svm, &token_account)
        {
            account
        } else {
            return Err(ClmmTestError::TokenAccountNotFound.into());
        };

        Ok(token_account.data)
    }
}
