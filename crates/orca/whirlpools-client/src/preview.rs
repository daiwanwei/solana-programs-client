use thiserror::Error;

use crate::{
    quote::{swap_quote_by_input_token, swap_quote_by_output_token},
    types::{PreviewSwapParams, PreviewSwapResult, TickArrayFacade},
};

pub fn preview_swap(params: PreviewSwapParams) -> Result<PreviewSwapResult> {
    let PreviewSwapParams {
        whirlpool,
        tick_arrays,
        amount,
        is_base_input,
        a_to_b,
        slippage_tolerance,
    } = params;

    let tick_arrays: Vec<TickArrayFacade> =
        tick_arrays.into_iter().map(|tick_array| tick_array.into()).collect();

    let transfer_fee_a = None;
    let transfer_fee_b = None;
    let specified_token_a = !(a_to_b || is_base_input);

    if tick_arrays.len() != 3 {
        return Err(PreviewError::InvalidTickArraysLength.into());
    }

    let arrays = [tick_arrays[0], tick_arrays[1], tick_arrays[2]];

    let result = if is_base_input {
        let res = swap_quote_by_input_token(
            amount,
            specified_token_a,
            slippage_tolerance,
            whirlpool.into(),
            arrays.into(),
            transfer_fee_a,
            transfer_fee_b,
        )
        .unwrap();

        PreviewSwapResult {
            amount_in: res.token_in,
            amount_out: res.token_est_out,
            fee: res.trade_fee,
            threshold: res.token_min_out,
        }
    } else {
        let res = swap_quote_by_output_token(
            amount,
            specified_token_a,
            slippage_tolerance,
            whirlpool.into(),
            arrays.into(),
            transfer_fee_a,
            transfer_fee_b,
        )
        .unwrap();

        PreviewSwapResult {
            amount_in: res.token_est_in,
            amount_out: res.token_out,
            fee: res.trade_fee,
            threshold: res.token_max_in,
        }
    };

    Ok(result)
}

#[derive(Error, Debug)]
pub enum PreviewError {
    #[error("invalid tick arrays length")]
    InvalidTickArraysLength,
}

pub type Result<T> = std::result::Result<T, PreviewError>;
