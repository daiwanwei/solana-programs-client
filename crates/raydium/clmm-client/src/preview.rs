use crate::{
    error::{ClmmClientError, Result},
    types::PreviewSwapV2Params,
};

pub fn preview_swap_v2(
    params: PreviewSwapV2Params,
) -> Result<raydium_clmm::math::swap_v2::SwapState> {
    let PreviewSwapV2Params {
        amount,
        sqrt_price_limit_x64,
        is_base_input,
        zero_for_one,
        protocol_fee_rate,
        pool_state,
        tick_array_bitmap,
        mut tick_array_accounts,
    } = params;

    let start_tick_index =
        tick_array_accounts.front().ok_or(ClmmClientError::NoTickArrayAvailable)?.start_tick_index;

    let (swap_state, _) = raydium_clmm::math::swap_v2::compute_swap(
        zero_for_one,
        is_base_input,
        true,
        protocol_fee_rate,
        amount,
        start_tick_index,
        sqrt_price_limit_x64,
        &pool_state,
        &tick_array_bitmap,
        &mut tick_array_accounts,
    )?;

    Ok(swap_state)
}
