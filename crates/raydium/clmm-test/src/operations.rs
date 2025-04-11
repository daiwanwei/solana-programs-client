use litesvm::LiteSVM;
use program_test_utils::account::get_anchor_account;
use raydium_clmm::{state, utils::derive};
use rust_decimal::Decimal;
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
    pub fn get_pool_state(&self, svm: &LiteSVM) -> Result<state::PoolState> {
        get_anchor_account::<state::PoolState>(svm, &self.pool_state)
            .ok_or(ClmmTestError::PoolStateNotFound.into())
            .map(|account| account.data)
    }

    pub fn get_tick_array_bitmap(&self, svm: &LiteSVM) -> Result<state::TickArrayBitmapExtension> {
        get_anchor_account::<state::TickArrayBitmapExtension>(svm, &self.tick_array_bitmap)
            .ok_or(ClmmTestError::TickArrayBitmapNotFound.into())
            .map(|account| account.data)
    }

    pub fn get_personal_position_state(
        &self,
        svm: &LiteSVM,
        position_nft_mint: Pubkey,
    ) -> Result<state::PersonalPositionState> {
        let personal_position =
            derive::derive_personal_position_pubkey(position_nft_mint, Some(self.program_id)).0;
        get_anchor_account::<state::PersonalPositionState>(&svm, &personal_position)
            .ok_or(ClmmTestError::PersonalPositionNotFound.into())
            .map(|account| account.data)
    }

    pub fn get_price(&self, svm: &LiteSVM) -> Result<Decimal> {
        let pool_state = self.get_pool_state(svm)?;
        Ok(raydium_clmm_client::math::price::calculate_price(
            pool_state.sqrt_price_x64,
            self.token_pair.decimals_0,
            self.token_pair.decimals_1,
        ))
    }
}
