use litesvm::{types::TransactionMetadata, LiteSVM};
use program_test_utils::{
    account::get_solana_account_by_pack, sign_and_send_transaction, token::create_mint,
};
use solana_client_core::types::MaybeAccount;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_token::ID as SPL_TOKEN_ID;

use crate::{
    error::{ClmmTestError, Result},
    types::{CreateAmmConfigParams, CreateMintsParams, CreatePoolParams},
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

    let mint_0_account = if let MaybeAccount::Exists(account) =
        get_solana_account_by_pack::<spl_token::state::Mint>(svm, &mint_0)
    {
        account
    } else {
        return Err(ClmmTestError::MintNotFound.into());
    };
    let mint_1_account = if let MaybeAccount::Exists(account) =
        get_solana_account_by_pack::<spl_token::state::Mint>(svm, &mint_1)
    {
        account
    } else {
        return Err(ClmmTestError::MintNotFound.into());
    };

    Ok((
        mint_0,
        mint_1,
        mint_0_account.data.decimals,
        mint_1_account.data.decimals,
        mint_0_account.account.owner,
        mint_1_account.account.owner,
    ))
}

pub fn create_amm_config(
    svm: &mut LiteSVM,
    payer: &Keypair,
    program_id: Pubkey,
    params: CreateAmmConfigParams,
) -> Result<(Pubkey, TransactionMetadata)> {
    let admin = crate::constants::ADMIN_KEY;
    let _unused = svm.airdrop(&admin, 1_000_000_000).unwrap();

    let (instruction, amm_config) =
        raydium_clmm_client::instructions::prepare_amm_config_instruction(
            raydium_clmm_client::types::CreateAmmConfigParams {
                owner: admin,
                config_index: params.config_index,
                tick_spacing: params.tick_spacing,
                trade_fee_rate: params.trade_fee_rate,
                protocol_fee_rate: params.protocol_fee_rate,
                fund_fee_rate: params.fund_fee_rate,
            },
            program_id,
        )?;

    let metadata = sign_and_send_transaction!(svm, &[instruction], payer)?;

    Ok((amm_config, metadata))
}

pub fn create_pool(
    svm: &mut LiteSVM,
    admin: &Keypair,
    program_id: Pubkey,
    mint0: Pubkey,
    mint1: Pubkey,
    amm_config: Pubkey,
    params: CreatePoolParams,
) -> Result<(Pubkey, TransactionMetadata)> {
    let (instruction, pool_state) =
        raydium_clmm_client::instructions::prepare_create_pool_instruction(
            raydium_clmm_client::types::CreatePoolParams {
                pool_creator: admin.pubkey(),
                amm_config,
                mint0,
                mint1,
                token_program0: SPL_TOKEN_ID,
                token_program1: SPL_TOKEN_ID,
                sqrt_price_x64: params.sqrt_price_x64,
                open_time: params.open_time,
            },
            program_id,
        )?;

    let metadata = sign_and_send_transaction!(svm, &[instruction], admin)?;

    Ok((pool_state, metadata))
}
