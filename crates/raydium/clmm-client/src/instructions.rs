use raydium_clmm::{generated, math::tick::get_array_start_index, utils::derive};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program, sysvar,
};
use spl_associated_token_account::{
    get_associated_token_address, ID as SPL_ASSOCIATED_TOKEN_ACCOUNT_ID,
};
use spl_token::ID as SPL_TOKEN_ID;
use spl_token_2022::ID as SPL_TOKEN_2022_ID;

use crate::{
    constants::{MEMO_PROGRAM_ID, METADATA_PROGRAM_ID},
    error::Result,
    types::*,
};

pub fn prepare_amm_config_instruction(
    params: CreateAmmConfigParams,
    program_id: Pubkey,
) -> Result<(Instruction, Pubkey)> {
    let amm_config = derive::derive_amm_config_pubkey(params.config_index, Some(program_id)).0;

    let ix = generated::instructions::CreateAmmConfig {
        owner: params.owner,
        amm_config,
        system_program: system_program::ID,
    };

    let args = generated::instructions::CreateAmmConfigInstructionArgs {
        index: params.config_index,
        tick_spacing: params.tick_spacing,
        trade_fee_rate: params.trade_fee_rate,
        protocol_fee_rate: params.protocol_fee_rate,
        fund_fee_rate: params.fund_fee_rate,
    };

    let mut instruction = ix.instruction(args);
    instruction.program_id = program_id;

    Ok((instruction, amm_config))
}

pub fn prepare_create_pool_instruction(
    params: CreatePoolParams,
    program_id: Pubkey,
) -> Result<(Instruction, Pubkey)> {
    let pool_state = derive::derive_pool_state_pubkey(
        params.amm_config,
        params.mint0,
        params.mint1,
        Some(program_id),
    )
    .0;
    let observation_state = derive::derive_observation_pubkey(pool_state, Some(program_id)).0;
    let tick_array_bitmap = derive::derive_tick_array_bitmap_pubkey(pool_state, Some(program_id)).0;
    let token_vault_0 =
        derive::derive_pool_vault_pubkey(pool_state, params.mint0, Some(program_id)).0;
    let token_vault_1 =
        derive::derive_pool_vault_pubkey(pool_state, params.mint1, Some(program_id)).0;

    let args = generated::instructions::CreatePoolInstructionArgs {
        sqrt_price_x64: params.sqrt_price_x64,
        open_time: params.open_time,
    };

    let ix = generated::instructions::CreatePool {
        pool_creator: params.pool_creator,
        amm_config: params.amm_config,
        pool_state,
        token_mint0: params.mint0,
        token_mint1: params.mint1,
        token_vault0: token_vault_0,
        token_vault1: token_vault_1,
        observation_state,
        tick_array_bitmap,
        token_program0: params.token_program0,
        token_program1: params.token_program1,
        system_program: system_program::ID,
        rent: sysvar::rent::ID,
    };

    let mut instruction = ix.instruction(args);
    instruction.program_id = program_id;

    Ok((instruction, pool_state))
}

pub fn prepare_open_position_v2_instruction(
    params: OpenPositionV2Params,
    program_id: Pubkey,
) -> Result<(Instruction, Keypair, Pubkey)> {
    let tick_array_lower_start_index = get_array_start_index(params.tick_lower_index, 1);
    let tick_array_upper_start_index = get_array_start_index(params.tick_upper_index, 1);

    let args = generated::instructions::OpenPositionV2InstructionArgs {
        tick_lower_index: params.tick_lower_index,
        tick_upper_index: params.tick_upper_index,
        tick_array_lower_start_index,
        tick_array_upper_start_index,
        liquidity: params.liquidity,
        amount0_max: params.amount0_max,
        amount1_max: params.amount1_max,
        with_metadata: params.with_metadata,
        base_flag: params.base_flag,
    };

    let position_nft_mint = Keypair::new();
    let personal_position =
        derive::derive_personal_position_pubkey(position_nft_mint.pubkey(), Some(program_id)).0;

    let protocol_position = derive::derive_protocol_position_pubkey(
        params.pool_state,
        params.tick_lower_index,
        params.tick_upper_index,
        Some(program_id),
    )
    .0;

    let tick_array_lower = derive::derive_tick_array_pubkey(
        params.pool_state,
        tick_array_lower_start_index,
        Some(program_id),
    )
    .0;
    let tick_array_upper = derive::derive_tick_array_pubkey(
        params.pool_state,
        tick_array_upper_start_index,
        Some(program_id),
    )
    .0;
    let position_nft_account =
        get_associated_token_address(&params.payer, &position_nft_mint.pubkey());

    let ix = generated::instructions::OpenPositionV2 {
        payer: params.payer,
        pool_state: params.pool_state,
        position_nft_owner: params.payer,
        position_nft_mint: position_nft_mint.pubkey(),
        position_nft_account,
        metadata_account: params.metadata_account,
        protocol_position,
        tick_array_lower,
        tick_array_upper,
        personal_position,
        token_account0: params.token_account0,
        token_account1: params.token_account1,
        token_vault0: params.token_vault0,
        token_vault1: params.token_vault1,
        token_program: SPL_TOKEN_ID,
        associated_token_program: SPL_ASSOCIATED_TOKEN_ACCOUNT_ID,
        metadata_program: METADATA_PROGRAM_ID,
        token_program2022: SPL_TOKEN_2022_ID,
        vault0_mint: params.mint0,
        vault1_mint: params.mint1,
        rent: sysvar::rent::ID,
        system_program: system_program::ID,
    };

    let tickarray_bitmap_extension =
        derive::derive_tick_array_bitmap_pubkey(params.pool_state, Some(program_id)).0;
    let remaining_accounts = vec![AccountMeta::new(tickarray_bitmap_extension, false)];

    let mut instruction = ix.instruction_with_remaining_accounts(args, &remaining_accounts);
    instruction.program_id = program_id;

    Ok((instruction, position_nft_mint, protocol_position))
}

pub fn prepare_increase_liquidity_v2_instruction(
    params: IncreaseLiquidityV2Params,
    program_id: Pubkey,
) -> Result<Instruction> {
    let args = generated::instructions::IncreaseLiquidityV2InstructionArgs {
        liquidity: params.liquidity,
        amount0_max: params.amount0_max,
        amount1_max: params.amount1_max,
        base_flag: Some(false),
    };

    let personal_position =
        derive::derive_personal_position_pubkey(params.position_nft_mint, Some(program_id)).0;
    let nft_account = get_associated_token_address(&params.nft_owner, &params.position_nft_mint);

    let ix = generated::instructions::IncreaseLiquidityV2 {
        nft_owner: params.nft_owner,
        pool_state: params.pool_state,
        nft_account,
        protocol_position: params.protocol_position,
        personal_position,
        tick_array_lower: params.tick_array_lower,
        tick_array_upper: params.tick_array_upper,
        token_account0: params.token_account0,
        token_account1: params.token_account1,
        token_vault0: params.token_vault0,
        token_vault1: params.token_vault1,
        token_program: SPL_TOKEN_ID,
        token_program2022: SPL_TOKEN_2022_ID,
        vault0_mint: params.mint0,
        vault1_mint: params.mint1,
    };

    let mut instruction = ix.instruction(args);
    instruction.program_id = program_id;

    Ok(instruction)
}

pub fn prepare_swap_v2_instruction(
    params: SwapV2Params,
    program_id: Pubkey,
) -> Result<Instruction> {
    let args = generated::instructions::SwapV2InstructionArgs {
        amount: params.amount,
        other_amount_threshold: params.other_amount_threshold,
        sqrt_price_limit_x64: params.sqrt_price_limit_x64,
        is_base_input: params.is_base_input,
    };

    let ix = generated::instructions::SwapV2 {
        payer: params.payer,
        amm_config: params.amm_config,
        pool_state: params.pool_state,
        input_token_account: params.input_token_account,
        output_token_account: params.output_token_account,
        input_vault: params.input_vault,
        output_vault: params.output_vault,
        observation_state: params.observation_state,
        token_program: SPL_TOKEN_ID,
        token_program2022: SPL_TOKEN_2022_ID,
        memo_program: MEMO_PROGRAM_ID,
        input_vault_mint: params.input_mint,
        output_vault_mint: params.output_mint,
    };

    let tick_array_bitmap_extension =
        derive::derive_tick_array_bitmap_pubkey(params.pool_state, Some(program_id)).0;

    let tick_array_bitmap_extension_account = AccountMeta::new(tick_array_bitmap_extension, false);

    let mut remaining_accounts = vec![tick_array_bitmap_extension_account];

    let tick_array_accounts = params
        .tick_array_accounts
        .into_iter()
        .map(|tick_array| AccountMeta::new(tick_array, false))
        .collect::<Vec<_>>();

    remaining_accounts.extend(tick_array_accounts);

    let mut instruction = ix.instruction_with_remaining_accounts(args, &remaining_accounts);
    instruction.program_id = program_id;

    Ok(instruction)
}

pub fn prepare_decrease_liquidity_v2_instruction(
    params: DecreaseLiquidityV2Params,
    program_id: Pubkey,
) -> Result<Instruction> {
    let args = generated::instructions::DecreaseLiquidityV2InstructionArgs {
        liquidity: params.liquidity,
        amount0_min: params.amount0_min,
        amount1_min: params.amount1_min,
    };

    let personal_position =
        derive::derive_personal_position_pubkey(params.position_nft_mint, Some(program_id)).0;
    let nft_account = get_associated_token_address(&params.nft_owner, &params.position_nft_mint);

    let ix = generated::instructions::DecreaseLiquidityV2 {
        nft_owner: params.nft_owner,
        nft_account,
        personal_position,
        pool_state: params.pool_state,
        protocol_position: params.protocol_position,
        token_vault0: params.token_vault0,
        token_vault1: params.token_vault1,
        tick_array_lower: params.tick_array_lower,
        tick_array_upper: params.tick_array_upper,
        recipient_token_account0: params.recipient_token_account0,
        recipient_token_account1: params.recipient_token_account1,
        token_program: SPL_TOKEN_ID,
        token_program2022: SPL_TOKEN_2022_ID,
        memo_program: MEMO_PROGRAM_ID,
        vault0_mint: params.mint0,
        vault1_mint: params.mint1,
    };

    let mut instruction = ix.instruction(args);
    instruction.program_id = program_id;

    Ok(instruction)
}
