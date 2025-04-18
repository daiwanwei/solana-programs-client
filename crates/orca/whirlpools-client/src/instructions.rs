use orca_whirlpools::utils::derive;
use solana_instruction_builder::prepare_anchor_ix;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program, sysvar,
};
use spl_associated_token_account::{
    get_associated_token_address, ID as SPL_ASSOCIATED_TOKEN_ACCOUNT_ID,
};
use spl_token::ID as SPL_TOKEN_ID;

use crate::{error::Result, types::*};

pub fn prepare_initialize_config_instruction(
    params: InitializeConfigParams,
    program_id: Pubkey,
) -> Result<(Instruction, Keypair)> {
    let whirlpool_config = Keypair::new();

    let ix = orca_whirlpools::instructions::InitializeConfig {
        fee_authority: params.fee_authority,
        collect_protocol_fees_authority: params.collect_protocol_fees_authority,
        reward_emissions_super_authority: params.reward_emissions_super_authority,
        default_protocol_fee_rate: params.default_protocol_fee_rate,
    };

    let accounts = orca_whirlpools::accounts::InitializeConfig {
        config: whirlpool_config.pubkey(),
        funder: params.owner,
        system_program: system_program::ID,
    };

    let instruction = prepare_anchor_ix!(program_id, ix, accounts);

    Ok((instruction, whirlpool_config))
}

pub fn prepare_initialize_fee_tier_instruction(
    params: InitializeFeeTierParams,
    program_id: Pubkey,
) -> Result<(Instruction, Pubkey)> {
    let (fee_tier, _) = derive::derive_fee_tier_pubkey(
        params.whirlpool_config,
        params.tick_spacing,
        Some(program_id),
    );

    let ix = orca_whirlpools::instructions::InitializeFeeTier {
        tick_spacing: params.tick_spacing,
        default_fee_rate: params.default_fee_rate,
    };

    let accounts = orca_whirlpools::accounts::InitializeFeeTier {
        fee_tier,
        funder: params.funder,
        config: params.whirlpool_config,
        fee_authority: params.fee_authority,
        system_program: system_program::ID,
    };

    let instruction = prepare_anchor_ix!(program_id, ix, accounts);

    Ok((instruction, fee_tier))
}

pub fn prepare_initialize_tick_array_instruction(
    params: InitializeTickArrayParams,
    program_id: Pubkey,
) -> Result<(Instruction, Pubkey)> {
    let (tick_array, _) = derive::derive_tick_array_pubkey(
        params.whirlpool,
        params.start_tick_index,
        Some(program_id),
    );

    let ix = orca_whirlpools::instructions::InitializeTickArray {
        start_tick_index: params.start_tick_index,
    };

    let accounts = orca_whirlpools::accounts::InitializeTickArray {
        whirlpool: params.whirlpool,
        funder: params.funder,
        tick_array,
        system_program: system_program::ID,
    };

    let instruction = prepare_anchor_ix!(program_id, ix, accounts);

    Ok((instruction, tick_array))
}

pub fn prepare_initialize_pool_instruction(
    params: InitializePoolParams,
    program_id: Pubkey,
) -> Result<(Instruction, Pubkey, Keypair, Keypair)> {
    let (whirlpool, whirlpool_bump) = derive::derive_whirlpool_pubkey(
        params.whirlpool_config,
        params.mint_a,
        params.mint_b,
        params.tick_spacing,
        Some(program_id),
    );

    let token_vault_a = Keypair::new();
    let token_vault_b = Keypair::new();

    let ix = orca_whirlpools::instructions::InitializePool {
        bumps: orca_whirlpools::instructions::WhirlpoolBumps { whirlpool_bump },
        tick_spacing: params.tick_spacing,
        initial_sqrt_price: params.sqrt_price,
    };

    let accounts = orca_whirlpools::accounts::InitializePool {
        whirlpools_config: params.whirlpool_config,
        funder: params.whirlpool_creator,
        whirlpool,
        token_mint_a: params.mint_a,
        token_mint_b: params.mint_b,
        token_vault_a: token_vault_a.pubkey(),
        token_vault_b: token_vault_b.pubkey(),
        fee_tier: params.fee_tier,
        token_program: SPL_TOKEN_ID,
        system_program: system_program::ID,
        rent: sysvar::rent::ID,
    };

    let instruction = prepare_anchor_ix!(program_id, ix, accounts);

    Ok((instruction, whirlpool, token_vault_a, token_vault_b))
}

pub fn prepare_open_position_instruction(
    params: OpenPositionParams,
    program_id: Pubkey,
) -> Result<(Instruction, Keypair)> {
    let position_mint = Keypair::new();
    let (position, position_bump) =
        derive::derive_position_pubkey(position_mint.pubkey(), Some(program_id));

    let ix = orca_whirlpools::instructions::OpenPosition {
        bumps: orca_whirlpools::instructions::OpenPositionBumps { position_bump },
        tick_lower_index: params.tick_lower_index,
        tick_upper_index: params.tick_upper_index,
    };

    let accounts = orca_whirlpools::accounts::OpenPosition {
        funder: params.payer,
        owner: params.owner,
        position,
        position_mint: position_mint.pubkey(),
        position_token_account: get_associated_token_address(
            &params.owner,
            &position_mint.pubkey(),
        ),
        whirlpool: params.whirlpool,
        token_program: SPL_TOKEN_ID,
        system_program: system_program::ID,
        rent: sysvar::rent::ID,
        associated_token_program: SPL_ASSOCIATED_TOKEN_ACCOUNT_ID,
    };

    let instruction = prepare_anchor_ix!(program_id, ix, accounts);

    Ok((instruction, position_mint))
}

pub fn prepare_increase_liquidity_instruction(
    params: IncreaseLiquidityParams,
    program_id: Pubkey,
) -> Result<Instruction> {
    let (position, _) = derive::derive_position_pubkey(params.position_nft_mint, Some(program_id));

    let ix = orca_whirlpools::instructions::IncreaseLiquidity {
        liquidity_amount: params.liquidity,
        token_max_a: params.token_max_a,
        token_max_b: params.token_max_b,
    };

    let accounts = orca_whirlpools::accounts::ModifyLiquidity {
        whirlpool: params.whirlpool,
        token_program: SPL_TOKEN_ID,
        position_authority: params.nft_owner,
        position,
        position_token_account: get_associated_token_address(
            &params.nft_owner,
            &params.position_nft_mint,
        ),
        token_owner_account_a: params.token_account_a,
        token_owner_account_b: params.token_account_b,
        token_vault_a: params.token_vault_a,
        token_vault_b: params.token_vault_b,
        tick_array_lower: params.tick_array_lower,
        tick_array_upper: params.tick_array_upper,
    };

    let instruction = prepare_anchor_ix!(program_id, ix, accounts);

    Ok(instruction)
}

pub fn prepare_swap_instruction(params: SwapParams, program_id: Pubkey) -> Result<Instruction> {
    let ix = orca_whirlpools::instructions::Swap {
        amount: params.amount,
        other_amount_threshold: params.other_amount_threshold,
        sqrt_price_limit: params.sqrt_price_limit,
        amount_specified_is_input: params.amount_specified_is_input,
        a_to_b: params.a_to_b,
    };

    let accounts = orca_whirlpools::accounts::Swap {
        token_program: SPL_TOKEN_ID,
        token_authority: params.token_authority,
        whirlpool: params.whirlpool,
        token_owner_account_a: params.token_owner_account_a,
        token_vault_a: params.token_vault_a,
        token_owner_account_b: params.token_owner_account_b,
        token_vault_b: params.token_vault_b,
        tick_array_0: params.tick_array_0,
        tick_array_1: params.tick_array_1,
        tick_array_2: params.tick_array_2,
        oracle: derive::derive_oracle_pubkey(params.whirlpool, Some(program_id)).0,
    };

    let instruction = prepare_anchor_ix!(program_id, ix, accounts);

    Ok(instruction)
}
