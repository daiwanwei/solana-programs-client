use orca_whirlpools::{constants::TICK_ARRAY_SIZE, utils::derive};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use crate::{
    error::Result,
    instructions,
    types::{InitializeTickArrayParams, InitializeTickArraysParams},
};

pub fn prepare_initialize_tick_array_instructions(
    params: InitializeTickArraysParams,
    program_id: Pubkey,
) -> Result<Vec<(Instruction, Pubkey)>> {
    let ticks_in_array = params.tick_spacing * TICK_ARRAY_SIZE;
    let direction = if params.a_to_b { -1 } else { 1 };
    let mut result = Vec::with_capacity(params.array_count as usize);

    for i in 0..params.array_count {
        let (ix, tick_array) = instructions::prepare_initialize_tick_array_instruction(
            InitializeTickArrayParams {
                whirlpool: params.whirlpool,
                start_tick_index: params.start_tick_index + direction * ticks_in_array * i,
                funder: params.funder,
            },
            program_id,
        )?;
        result.push((ix, tick_array));
    }

    Ok(result)
}

pub fn get_tick_array_pubkeys(
    whirlpool: Pubkey,
    tick_start_index: i32,
    tick_spacing: u16,
    a_to_b: bool,
    count: u32,
    program_id: Pubkey,
) -> Vec<Pubkey> {
    let direction: i32 = if a_to_b { -1 } else { 1 };
    let mut result = Vec::with_capacity(count as usize);
    let ticks_in_array = tick_spacing as i32 * TICK_ARRAY_SIZE;

    for i in 0..count {
        let start_index = tick_start_index + direction * ticks_in_array * i as i32;
        println!("start_index: {}", start_index);
        let tick_array =
            derive::derive_tick_array_pubkey(whirlpool, start_index, Some(program_id)).0;
        result.push(tick_array);
    }

    result
}
