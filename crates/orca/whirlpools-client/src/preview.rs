use thiserror::Error;

use crate::{
    quote::{swap_quote_by_input_token, swap_quote_by_output_token},
    types::TickArrayFacade,
};

pub fn preview_swap(
    whirlpool: orca_whirlpools::generated::accounts::Whirlpool,
    tick_arrays: Vec<orca_whirlpools::generated::accounts::TickArray>,
    slippage_tolerance: u16,
    amount: u64,
    is_base_input: bool,
    a_to_b: bool,
) -> Result<(u64, u64, u64)> {
    let tick_arrays: Vec<TickArrayFacade> =
        tick_arrays.into_iter().map(|tick_array| tick_array.into()).collect();

    let transfer_fee_a = None;
    let transfer_fee_b = None;
    let specified_token_a = !(a_to_b || is_base_input);

    if tick_arrays.len() != 3 {
        return Err(PreviewError::InvalidTickArraysLength.into());
    }

    let arrays = [tick_arrays[0], tick_arrays[1], tick_arrays[2]];

    let (amount_in, amount_out, threshold) = if is_base_input {
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

        (res.token_in, res.token_est_out, res.token_min_out)
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

        (res.token_est_in, res.token_out, res.token_max_in)
    };

    Ok((amount_in, amount_out, threshold))
}

#[derive(Error, Debug)]
pub enum PreviewError {
    #[error("invalid tick arrays length")]
    InvalidTickArraysLength,
}

pub type Result<T> = std::result::Result<T, PreviewError>;
