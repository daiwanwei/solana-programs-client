use std::collections::VecDeque;

use litesvm::{types::TransactionMetadata, LiteSVM};
use program_test_utils::sign_and_send_transaction;
use raydium_clmm::{math::swap_v2::SwapState, utils::derive};
use raydium_clmm_client::{preview::preview_swap_v2, types::PreviewSwapV2Params};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

use crate::{
    error::Result,
    operations::RaydiumClmmTest,
    types::{
        DecreaseLiquidityV2Params, IncreaseLiquidityV2Params, OpenPositionV2Params, SwapV2Params,
    },
};

impl RaydiumClmmTest {
    pub fn open_position_v2(
        &self,
        svm: &mut LiteSVM,
        params: OpenPositionV2Params,
        signer: &Keypair,
    ) -> Result<(Pubkey, Pubkey, TransactionMetadata)> {
        let (instruction, position_nft_mint, protocol_position) =
            raydium_clmm_client::instructions::prepare_open_position_v2_instruction(
                raydium_clmm_client::types::OpenPositionV2Params {
                    pool_state: self.pool_state,
                    mint0: self.token_pair.mint0,
                    mint1: self.token_pair.mint1,
                    token_vault0: self.token_vault0,
                    token_vault1: self.token_vault1,
                    payer: signer.pubkey(),
                    token_account0: params.user_token_account0,
                    token_account1: params.user_token_account1,
                    tick_lower_index: params.tick_lower_index,
                    tick_upper_index: params.tick_upper_index,
                    liquidity: params.liquidity,
                    amount0_max: params.amount0_max,
                    amount1_max: params.amount1_max,
                    with_metadata: false,
                    base_flag: Some(false),
                    metadata_account: Pubkey::default(),
                },
                self.program_id,
            )?;
        let metadata = sign_and_send_transaction!(svm, &[instruction], signer)?;

        Ok((position_nft_mint.pubkey(), protocol_position, metadata))
    }

    pub fn increase_liquidity_v2(
        &self,
        svm: &mut LiteSVM,
        params: IncreaseLiquidityV2Params,
        signer: &Keypair,
    ) -> Result<TransactionMetadata> {
        let personal_position_account =
            self.get_personal_position_state(svm, params.position_nft_mint)?;

        let protocol_position = derive::derive_protocol_position_pubkey(
            self.pool_state,
            personal_position_account.tick_lower_index,
            personal_position_account.tick_upper_index,
            Some(self.program_id),
        )
        .0;

        let tick_array_lower = derive::derive_tick_array_pubkey_by_tick_index(
            self.pool_state,
            personal_position_account.tick_lower_index,
            self.fee_config.tick_spacing,
            Some(self.program_id),
        )
        .0;
        let tick_array_upper = derive::derive_tick_array_pubkey_by_tick_index(
            self.pool_state,
            personal_position_account.tick_upper_index,
            self.fee_config.tick_spacing,
            Some(self.program_id),
        )
        .0;

        let instruction =
            raydium_clmm_client::instructions::prepare_increase_liquidity_v2_instruction(
                raydium_clmm_client::types::IncreaseLiquidityV2Params {
                    pool_state: self.pool_state,
                    position_nft_mint: params.position_nft_mint,
                    token_account0: params.user_token_account0,
                    token_account1: params.user_token_account1,
                    token_vault0: self.token_vault0,
                    token_vault1: self.token_vault1,
                    nft_owner: signer.pubkey(),
                    mint0: self.token_pair.mint0,
                    mint1: self.token_pair.mint1,
                    protocol_position,
                    tick_array_lower,
                    tick_array_upper,
                    liquidity: params.liquidity,
                    amount0_max: params.amount0_max,
                    amount1_max: params.amount1_max,
                },
                self.program_id,
            )?;

        let metadata = sign_and_send_transaction!(svm, &[instruction], signer)?;

        Ok(metadata)
    }

    pub fn swap_v2(
        &self,
        svm: &mut LiteSVM,
        params: SwapV2Params,
        signer: &Keypair,
    ) -> Result<TransactionMetadata> {
        let (
            input_mint,
            output_mint,
            input_vault,
            output_vault,
            input_token_account,
            output_token_account,
        ) = if params.zero_for_one {
            (
                self.token_pair.mint0,
                self.token_pair.mint1,
                self.token_vault0,
                self.token_vault1,
                params.user_token_account0,
                params.user_token_account1,
            )
        } else {
            (
                self.token_pair.mint1,
                self.token_pair.mint0,
                self.token_vault1,
                self.token_vault0,
                params.user_token_account1,
                params.user_token_account0,
            )
        };

        let pool_state_account = self.get_pool_state(svm)?;
        let tick_array_bitmap_extension = self.get_tick_array_bitmap(svm)?;

        let tick_array_accounts =
            raydium_clmm_client::utils::tick_array::load_cur_and_next_five_tick_array_pubkey(
                self.pool_state,
                &pool_state_account,
                &tick_array_bitmap_extension,
                params.zero_for_one,
                Some(self.program_id),
            );

        let instruction = raydium_clmm_client::instructions::prepare_swap_v2_instruction(
            raydium_clmm_client::types::SwapV2Params {
                amount: params.amount,
                other_amount_threshold: params.other_amount_threshold,
                sqrt_price_limit_x64: params.sqrt_price_limit_x64,
                is_base_input: params.is_base_input,
                zero_for_one: params.zero_for_one,
                amm_config: self.amm_config,
                pool_state: self.pool_state,
                input_token_account,
                output_token_account,
                input_vault,
                output_vault,
                observation_state: self.observation_state,
                input_mint,
                output_mint,
                payer: signer.pubkey(),
                tick_array_accounts,
            },
            self.program_id,
        )?;

        let metadata = sign_and_send_transaction!(svm, &[instruction], signer)?;

        Ok(metadata)
    }

    pub fn preview_swap_v2(&self, svm: &LiteSVM, params: SwapV2Params) -> Result<SwapState> {
        let pool_state_account = self.get_pool_state(svm)?;
        let amm_config = self.get_amm_config(svm)?;
        let tick_array_bitmap_extension = self.get_tick_array_bitmap(svm)?;

        let tick_array_accounts =
            raydium_clmm_client::utils::tick_array::load_cur_and_next_five_tick_array_pubkey(
                self.pool_state,
                &pool_state_account,
                &tick_array_bitmap_extension,
                params.zero_for_one,
                Some(self.program_id),
            );

        let mut tick_arrays = VecDeque::new();
        for tick_array_account in tick_array_accounts {
            let tick_array = self.get_tick_array(svm, tick_array_account)?;
            tick_arrays.push_back(tick_array.clone());
        }

        let swap_state = preview_swap_v2(PreviewSwapV2Params {
            amount: params.amount,
            sqrt_price_limit_x64: params.sqrt_price_limit_x64,
            is_base_input: params.is_base_input,
            zero_for_one: params.zero_for_one,
            trade_fee_rate: amm_config.trade_fee_rate,
            pool_state: pool_state_account,
            tick_array_bitmap: tick_array_bitmap_extension,
            tick_array_accounts: tick_arrays,
        })?;

        Ok(swap_state)
    }

    pub fn decrease_liquidity_v2(
        &self,
        svm: &mut LiteSVM,
        params: DecreaseLiquidityV2Params,
        signer: &Keypair,
    ) -> Result<TransactionMetadata> {
        let instruction =
            raydium_clmm_client::instructions::prepare_decrease_liquidity_v2_instruction(
                raydium_clmm_client::types::DecreaseLiquidityV2Params {
                    pool_state: self.pool_state,
                    position_nft_mint: params.position_nft_mint,
                    token_vault0: self.token_vault0,
                    token_vault1: self.token_vault1,
                    recipient_token_account0: params.recipient_token_account0,
                    recipient_token_account1: params.recipient_token_account1,
                    nft_owner: signer.pubkey(),
                    mint0: self.token_pair.mint0,
                    mint1: self.token_pair.mint1,
                    protocol_position: Pubkey::default(),
                    tick_array_lower: Pubkey::default(),
                    tick_array_upper: Pubkey::default(),
                    liquidity: params.liquidity,
                    amount0_min: params.amount0_min,
                    amount1_min: params.amount1_min,
                },
                self.program_id,
            )?;

        let metadata = sign_and_send_transaction!(svm, &[instruction], signer)?;

        Ok(metadata)
    }
}
