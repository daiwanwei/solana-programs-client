use borsh::BorshDeserialize;
use litesvm::{types::TransactionMetadata, LiteSVM};
use orca_whirlpools::utils::derive;
use program_test_utils::{
    account::{
        get_solana_account_by_borsh, get_solana_account_by_pack, get_solana_accounts_by_borsh,
    },
    sign_and_send_transaction,
};
use solana_client_core::MaybeAccount;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

use crate::{
    error::{Result, WhirlpoolsTestError},
    types::*,
};

pub struct WhirlpoolsTester {
    pub program_id: Pubkey,
    pub token_pair: TokenPair,
    pub whirlpool_config: Pubkey,
    pub whirlpool: Pubkey,
    pub fee_tier: Pubkey,
    pub token_vault_a: Pubkey,
    pub token_vault_b: Pubkey,
    pub tick_spacing: u16,
}

impl WhirlpoolsTester {
    pub fn new(params: CreateWhirlPoolTesterParams) -> Self {
        Self {
            program_id: params.program_id,
            token_pair: params.token_pair,
            whirlpool_config: params.whirlpool_config,
            whirlpool: params.whirlpool,
            fee_tier: params.fee_tier,
            token_vault_a: params.token_vault_a,
            token_vault_b: params.token_vault_b,
            tick_spacing: params.tick_spacing,
        }
    }
}

impl WhirlpoolsTester {
    pub fn get_program_account<T: BorshDeserialize>(
        &self,
        svm: &LiteSVM,
        account: &Pubkey,
    ) -> Result<T> {
        let account =
            if let MaybeAccount::Exists(account) = get_solana_account_by_borsh::<T>(svm, account) {
                account
            } else {
                return Err(WhirlpoolsTestError::ProgramAccountNotFound);
            };

        Ok(account.data)
    }

    pub fn get_program_accounts<T: BorshDeserialize>(
        &self,
        svm: &LiteSVM,
        accounts: &[Pubkey],
    ) -> Result<Vec<T>> {
        let accounts = get_solana_accounts_by_borsh::<T>(svm, accounts);

        Ok(accounts
            .into_iter()
            .filter_map(|account| {
                if let MaybeAccount::Exists(account) = account {
                    Some(account.data)
                } else {
                    None
                }
            })
            .collect())
    }

    pub fn get_whirlpool(
        &self,
        svm: &LiteSVM,
        whirlpool: &Pubkey,
    ) -> Result<orca_whirlpools::generated::accounts::Whirlpool> {
        self.get_program_account::<orca_whirlpools::generated::accounts::Whirlpool>(svm, whirlpool)
    }

    pub fn get_position(
        &self,
        svm: &LiteSVM,
        position: &Pubkey,
    ) -> Result<orca_whirlpools::generated::accounts::Position> {
        self.get_program_account::<orca_whirlpools::generated::accounts::Position>(svm, position)
    }

    pub fn get_tick_arrays(
        &self,
        svm: &LiteSVM,
        tick_arrays: &[Pubkey],
    ) -> Result<Vec<orca_whirlpools::generated::accounts::TickArray>> {
        self.get_program_accounts::<orca_whirlpools::generated::accounts::TickArray>(
            svm,
            tick_arrays,
        )
    }

    pub fn get_token_account(
        &self,
        svm: &LiteSVM,
        token_account: &Pubkey,
    ) -> Result<spl_token::state::Account> {
        let token_account_account = if let MaybeAccount::Exists(account) =
            get_solana_account_by_pack::<spl_token::state::Account>(svm, token_account)
        {
            account
        } else {
            return Err(WhirlpoolsTestError::TokenAccountNotFound);
        };

        Ok(token_account_account.data)
    }
}

impl WhirlpoolsTester {
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
                    whirlpool: self.whirlpool,
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
                whirlpool: self.whirlpool,
                position_nft_mint: params.position_nft_mint,
                tick_array_lower,
                tick_array_upper,
                token_account_a: params.token_account_a,
                token_account_b: params.token_account_b,
                token_vault_a: self.token_vault_a,
                token_vault_b: self.token_vault_b,
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
                whirlpool: self.whirlpool,
                token_owner_account_a: params.token_owner_account_a,
                token_vault_a: self.token_vault_a,
                token_owner_account_b: params.token_owner_account_b,
                token_vault_b: self.token_vault_b,
                tick_array0: tick_arrays[0],
                tick_array1: tick_arrays[1],
                tick_array2: tick_arrays[2],
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

    pub fn preview_swap(
        &self,
        svm: &LiteSVM,
        slippage_tolerance: u16,
        amount: u64,
        is_base_input: bool,
        a_to_b: bool,
    ) -> Result<orca_whirlpools_client::types::PreviewSwapResult> {
        let whirlpool = self.get_whirlpool(svm, &self.whirlpool)?;

        let start_tick_index = orca_whirlpools::math::tick::get_array_start_index(
            whirlpool.tick_current_index,
            whirlpool.tick_spacing,
        );

        let tick_arrays = orca_whirlpools_client::utils::tick_array::get_tick_array_pubkeys(
            self.whirlpool,
            start_tick_index,
            whirlpool.tick_spacing,
            a_to_b,
            3,
            self.program_id,
        );

        let tick_arrays = self.get_tick_arrays(svm, &tick_arrays)?;

        let result = orca_whirlpools_client::preview::preview_swap(
            orca_whirlpools_client::types::PreviewSwapParams {
                whirlpool,
                tick_arrays,
                amount,
                is_base_input,
                a_to_b,
                slippage_tolerance,
            },
        )
        .unwrap();

        Ok(result)
    }
}
