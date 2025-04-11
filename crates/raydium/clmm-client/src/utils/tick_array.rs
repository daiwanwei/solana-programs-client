use raydium_clmm::{
    state::{PoolState, TickArrayBitmapExtension},
    utils::derive::derive_tick_array_pubkey,
};
use solana_program::pubkey::Pubkey;

pub fn load_cur_and_next_five_tick_array_pubkey(
    pool_id: Pubkey,
    pool_state: &PoolState,
    tickarray_bitmap_extension: &TickArrayBitmapExtension,
    zero_for_one: bool,
    program_id: Option<Pubkey>,
) -> Vec<Pubkey> {
    let (_, mut current_vaild_tick_array_start_index) = pool_state
        .get_first_initialized_tick_array(&Some(tickarray_bitmap_extension.clone()), zero_for_one)
        .unwrap();
    let mut tick_array_keys = Vec::new();
    tick_array_keys.push(
        derive_tick_array_pubkey(pool_id, current_vaild_tick_array_start_index, program_id).0,
    );
    let mut max_array_size = 5;
    while max_array_size != 0 {
        let next_tick_array_index = pool_state
            .next_initialized_tick_array_start_index(
                &Some(tickarray_bitmap_extension.clone()),
                current_vaild_tick_array_start_index,
                zero_for_one,
            )
            .unwrap();
        if next_tick_array_index.is_none() {
            break;
        }
        current_vaild_tick_array_start_index = next_tick_array_index.unwrap();
        tick_array_keys.push(
            derive_tick_array_pubkey(pool_id, current_vaild_tick_array_start_index, program_id).0,
        );
        max_array_size -= 1;
    }
    tick_array_keys
}
