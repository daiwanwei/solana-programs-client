use litesvm::{types::TransactionMetadata, LiteSVM};
use program_test_utils::{
    account::get_solana_account, sign_and_send_transaction, token::create_mint,
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_token::ID as SPL_TOKEN_ID;

use crate::{
    error::{Result, WhirlpoolsTestError},
    types::{
        CreateConfigParams, CreateFeeTierParams, CreateMintsParams, InitializeFeeTierParams,
        InitializePoolParams, InitializeTickArraysParams,
    },
};

pub fn create_mints(
    svm: &mut LiteSVM,
    admin: &Keypair,
    params: CreateMintsParams,
) -> Result<(Pubkey, Pubkey, u8, u8, Pubkey, Pubkey)> {
    let token_program_id_a = params.token_program_id_a.unwrap_or(SPL_TOKEN_ID);
    let token_program_id_b = params.token_program_id_b.unwrap_or(SPL_TOKEN_ID);

    let (mint_a, _) =
        create_mint(svm, admin, &admin.pubkey(), params.decimals_a, token_program_id_a).unwrap();
    let (mint_b, _) =
        create_mint(svm, admin, &admin.pubkey(), params.decimals_b, token_program_id_b).unwrap();
    let (mint_0, mint_1, decimals_0, decimals_1, token_program_id_0, token_program_id_1) =
        if mint_a < mint_b {
            (
                mint_a,
                mint_b,
                params.decimals_a,
                params.decimals_b,
                token_program_id_a,
                token_program_id_b,
            )
        } else {
            (
                mint_b,
                mint_a,
                params.decimals_b,
                params.decimals_a,
                token_program_id_b,
                token_program_id_a,
            )
        };

    Ok((mint_0, mint_1, decimals_0, decimals_1, token_program_id_0, token_program_id_1))
}

pub fn get_mints(
    svm: &LiteSVM,
    mint_a: Pubkey,
    mint_b: Pubkey,
) -> Result<(Pubkey, Pubkey, u8, u8, Pubkey, Pubkey)> {
    let (mint_0, mint_1) = if mint_a < mint_b { (mint_a, mint_b) } else { (mint_b, mint_a) };

    let mint_0_account = get_solana_account::<spl_token::state::Mint>(svm, &mint_0)
        .ok_or(WhirlpoolsTestError::MintNotFound)?;
    let mint_1_account = get_solana_account::<spl_token::state::Mint>(svm, &mint_1)
        .ok_or(WhirlpoolsTestError::MintNotFound)?;

    Ok((
        mint_0,
        mint_1,
        mint_0_account.data.decimals,
        mint_1_account.data.decimals,
        mint_0_account.owner,
        mint_1_account.owner,
    ))
}

pub fn create_config(
    svm: &mut LiteSVM,
    payer: &Keypair,
    program_id: Pubkey,
    params: CreateConfigParams,
) -> Result<(Pubkey, TransactionMetadata)> {
    let (instruction, config) =
        orca_whirlpools_client::instructions::prepare_initialize_config_instruction(
            orca_whirlpools_client::types::InitializeConfigParams {
                owner: payer.pubkey(),
                fee_authority: payer.pubkey(),
                collect_protocol_fees_authority: payer.pubkey(),
                reward_emissions_super_authority: payer.pubkey(),
                default_protocol_fee_rate: params.default_protocol_fee_rate,
            },
            program_id,
        )?;

    let metadata = sign_and_send_transaction!(svm, &[instruction], payer, &[&config])?;

    Ok((config.pubkey(), metadata))
}

pub fn initialize_fee_tier(
    svm: &mut LiteSVM,
    payer: &Keypair,
    program_id: Pubkey,
    params: InitializeFeeTierParams,
) -> Result<(Pubkey, TransactionMetadata)> {
    let (instruction, fee_tier) =
        orca_whirlpools_client::instructions::prepare_initialize_fee_tier_instruction(
            orca_whirlpools_client::types::InitializeFeeTierParams {
                funder: payer.pubkey(),
                whirlpool_config: params.whirlpool_config,
                fee_authority: payer.pubkey(),
                tick_spacing: params.tick_spacing,
                default_fee_rate: params.default_fee_rate,
            },
            program_id,
        )?;

    let metadata = sign_and_send_transaction!(svm, &[instruction], payer)?;

    Ok((fee_tier, metadata))
}

pub fn initialize_pool(
    svm: &mut LiteSVM,
    admin: &Keypair,
    program_id: Pubkey,
    params: InitializePoolParams,
) -> Result<(Pubkey, Pubkey, Pubkey, TransactionMetadata)> {
    let (instruction, whirlpool, token_vault_a, token_vault_b) =
        orca_whirlpools_client::instructions::prepare_initialize_pool_instruction(
            orca_whirlpools_client::types::InitializePoolParams {
                whirlpool_creator: admin.pubkey(),
                whirlpool_config: params.whirlpool_config,
                fee_tier: params.fee_tier,
                mint_a: params.mint_a,
                mint_b: params.mint_b,
                tick_spacing: params.tick_spacing,
                sqrt_price: params.sqrt_price,
            },
            program_id,
        )?;

    let metadata =
        sign_and_send_transaction!(svm, &[instruction], admin, &[&token_vault_a, &token_vault_b])?;

    Ok((whirlpool, token_vault_a.pubkey(), token_vault_b.pubkey(), metadata))
}

pub fn initialize_tick_arrays(
    svm: &mut LiteSVM,
    payer: &Keypair,
    program_id: Pubkey,
    params: InitializeTickArraysParams,
) -> Result<TransactionMetadata> {
    let instructions =
        orca_whirlpools_client::utils::tick_array::prepare_initialize_tick_array_instructions(
            orca_whirlpools_client::types::InitializeTickArraysParams {
                whirlpool: params.whirlpool,
                funder: payer.pubkey(),
                start_tick_index: params.start_tick_index,
                array_count: params.array_count as i32,
                tick_spacing: params.tick_spacing as i32,
                a_to_b: params.a_to_b,
            },
            program_id,
        )?
        .into_iter()
        .map(|(instruction, _)| instruction)
        .collect::<Vec<_>>();

    let metadata = sign_and_send_transaction!(svm, &instructions, payer)?;

    Ok(metadata)
}
