use litesvm::{types::TransactionMetadata, LiteSVM};
use orca_whirlpools::utils::derive;
use program_test_utils::sign_and_send_transaction;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

use crate::{
    error::{Result, WhirlpoolsTestError},
    operations::WhirlpoolsTest,
    types::*,
};

impl WhirlpoolsTest {
    pub fn open_position(
        &self,
        svm: &mut LiteSVM,
        payer: &Keypair,
        params: OpenPositionParams,
    ) -> Result<(Pubkey, TransactionMetadata)> {
        let (instruction, position_nft_mint) =
            orca_whirlpools_client::instructions::prepare_open_position_instruction(
                orca_whirlpools_client::types::OpenPositionParams {
                    payer: payer.pubkey(),
                    owner: params.owner,
                    whirlpool: params.whirlpool,
                    tick_lower_index: params.tick_lower_index,
                    tick_upper_index: params.tick_upper_index,
                },
                self.program_id,
            )?;

        let metadata =
            sign_and_send_transaction!(svm, &[instruction], payer, &[&position_nft_mint])?;

        Ok((position_nft_mint.pubkey(), metadata))
    }

    pub fn increase_liquidity(
        &self,
        svm: &mut LiteSVM,
        payer: &Keypair,
        params: IncreaseLiquidityParams,
    ) -> Result<TransactionMetadata> {
        let position =
            derive::derive_position_pubkey(params.position_nft_mint, Some(self.program_id)).0;

        let position_account = self.get_position(svm, &position)?;

        let tick_array_lower = derive::derive_tick_array_pubkey(
            self.whirlpool,
            orca_whirlpools::math::tick::get_array_start_index(
                position_account.tick_lower_index,
                self.tick_spacing,
            ),
            Some(self.program_id),
        )
        .0;

        let tick_array_upper = derive::derive_tick_array_pubkey(
            self.whirlpool,
            orca_whirlpools::math::tick::get_array_start_index(
                position_account.tick_upper_index,
                self.tick_spacing,
            ),
            Some(self.program_id),
        )
        .0;

        let ix = orca_whirlpools_client::instructions::prepare_increase_liquidity_instruction(
            orca_whirlpools_client::types::IncreaseLiquidityParams {
                nft_owner: params.nft_owner,
                whirlpool: params.whirlpool,
                position_nft_mint: params.position_nft_mint,
                tick_array_lower,
                tick_array_upper,
                token_account_a: params.token_account_a,
                token_account_b: params.token_account_b,
                token_vault_a: params.token_vault_a,
                token_vault_b: params.token_vault_b,
                mint_a: self.token_pair.mint_a,
                mint_b: self.token_pair.mint_b,
                liquidity: params.liquidity,
                token_max_a: params.token_max_a,
                token_max_b: params.token_max_b,
            },
            self.program_id,
        )?;

        let metadata = sign_and_send_transaction!(svm, &[ix], payer)?;

        Ok(metadata)
    }

    // pub fn decrease_liquidity(
    //     &self,
    //     svm: &mut LiteSVM,
    //     payer: &Keypair,
    //     params: DecreaseLiquidityParams,
    // ) -> Result<TransactionMetadata> {
    //     let ix =
    // orca_whirlpools_client::instructions::prepare_decrease_liquidity_instruction(
    //         orca_whirlpools_client::types::DecreaseLiquidityParams {
    //             nft_owner: params.nft_owner,
    //             whirlpool: params.whirlpool,
    //             position_nft_mint: params.position_nft_mint,
    //             position: params.position,
    //             tick_array_lower: params.tick_array_lower,
    //             tick_array_upper: params.tick_array_upper,
    //             token_vault_a: params.token_vault_a,
    //             token_vault_b: params.token_vault_b,
    //             recipient_token_account_a: params.recipient_token_account_a,
    //             recipient_token_account_b: params.recipient_token_account_b,
    //             liquidity: params.liquidity,
    //             token_min_a: params.token_min_a,
    //             token_min_b: params.token_min_b,
    //         },
    //         self.program_id,
    //     )?;

    //     let metadata = sign_and_send_transaction!(svm, &[ix], payer)?;

    //     Ok(metadata)
    // }

    pub fn swap(
        &self,
        svm: &mut LiteSVM,
        payer: &Keypair,
        params: SwapParams,
    ) -> Result<TransactionMetadata> {
        let whirlpool_account = self.get_whirlpool(svm, &self.whirlpool)?;

        let start_tick_index = orca_whirlpools::math::tick::get_array_start_index(
            whirlpool_account.tick_current_index,
            whirlpool_account.tick_spacing,
        );

        let tick_arrays = orca_whirlpools_client::utils::tick_array::get_tick_array_pubkeys(
            self.whirlpool,
            start_tick_index,
            whirlpool_account.tick_spacing,
            params.a_to_b,
            3,
            self.program_id,
        );

        if tick_arrays.len() != 3 {
            return Err(WhirlpoolsTestError::InvalidTickArrays);
        }

        let ix = orca_whirlpools_client::instructions::prepare_swap_instruction(
            orca_whirlpools_client::types::SwapParams {
                token_authority: params.token_authority,
                whirlpool: params.whirlpool,
                token_owner_account_a: params.token_owner_account_a,
                token_vault_a: params.token_vault_a,
                token_owner_account_b: params.token_owner_account_b,
                token_vault_b: params.token_vault_b,
                tick_array_0: tick_arrays[0],
                tick_array_1: tick_arrays[1],
                tick_array_2: tick_arrays[2],
                amount: params.amount,
                other_amount_threshold: params.other_amount_threshold,
                sqrt_price_limit: params.sqrt_price_limit,
                amount_specified_is_input: params.amount_specified_is_input,
                a_to_b: params.a_to_b,
            },
            self.program_id,
        )?;

        let metadata = sign_and_send_transaction!(svm, &[ix], payer).inspect_err(|e| {
            println!("Error swapping: {:?}", e);
        })?;

        Ok(metadata)
    }
}
