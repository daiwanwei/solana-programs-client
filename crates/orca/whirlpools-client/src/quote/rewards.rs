use ethnum::U256;

use crate::{
    constants::{CoreError, AMOUNT_EXCEEDS_MAX_U64, ARITHMETIC_OVERFLOW, NUM_REWARDS},
    math::try_apply_transfer_fee,
    types::{
        CollectRewardQuote, CollectRewardsQuote, PositionFacade, TickFacade, TransferFee,
        WhirlpoolFacade,
    },
};

/// Calculate rewards owed for a position
///
/// # Paramters
/// - `whirlpool`: The whirlpool state
/// - `position`: The position state
/// - `tick_lower`: The lower tick state
/// - `tick_upper`: The upper tick state
/// - `current_timestamp`: The current timestamp
/// - `transfer_fee_1`: The transfer fee for token 1
/// - `transfer_fee_2`: The transfer fee for token 2
/// - `transfer_fee_3`: The transfer fee for token 3
///
/// # Returns
/// - `CollectRewardsQuote`: The rewards owed for the 3 reward tokens.
#[allow(clippy::too_many_arguments)]
pub fn collect_rewards_quote(
    whirlpool: WhirlpoolFacade,
    position: PositionFacade,
    tick_lower: TickFacade,
    tick_upper: TickFacade,
    current_timestamp: u64,
    transfer_fee_1: Option<TransferFee>,
    transfer_fee_2: Option<TransferFee>,
    transfer_fee_3: Option<TransferFee>,
) -> Result<CollectRewardsQuote, CoreError> {
    let timestamp_delta = current_timestamp - whirlpool.reward_last_updated_timestamp;
    let transfer_fees = [transfer_fee_1, transfer_fee_2, transfer_fee_3];
    let mut reward_quotes: [CollectRewardQuote; NUM_REWARDS] =
        [CollectRewardQuote::default(); NUM_REWARDS];

    for i in 0..NUM_REWARDS {
        let mut reward_growth: u128 = whirlpool.reward_infos[i].growth_global_x64;
        if whirlpool.liquidity != 0 {
            let reward_growth_delta = whirlpool.reward_infos[i]
                .emissions_per_second_x64
                .checked_mul(timestamp_delta as u128)
                .ok_or(ARITHMETIC_OVERFLOW)?
                / whirlpool.liquidity;
            reward_growth += <u128>::try_from(reward_growth_delta).unwrap();
        }
        let mut reward_growth_below = tick_lower.reward_growths_outside[i];
        let mut reward_growth_above = tick_upper.reward_growths_outside[i];

        if whirlpool.tick_current_index < position.tick_lower_index {
            reward_growth_below = reward_growth.wrapping_sub(reward_growth_below);
        }

        if whirlpool.tick_current_index >= position.tick_upper_index {
            reward_growth_above = reward_growth.wrapping_sub(reward_growth_above);
        }

        let reward_growth_inside =
            reward_growth.wrapping_sub(reward_growth_below).wrapping_sub(reward_growth_above);

        let reward_growth_delta =
            reward_growth_inside.wrapping_sub(position.reward_infos[i].growth_inside_checkpoint);

        let reward_owed_delta: u64 = <U256>::from(reward_growth_delta)
            .checked_mul(position.liquidity.into())
            .ok_or(ARITHMETIC_OVERFLOW)?
            .try_into()
            .map_err(|_| AMOUNT_EXCEEDS_MAX_U64)?;

        let withdrawable_reward = position.reward_infos[i].amount_owed + reward_owed_delta;
        let rewards_owed =
            try_apply_transfer_fee(withdrawable_reward, transfer_fees[i].unwrap_or_default())?;
        reward_quotes[i] = CollectRewardQuote { rewards_owed }
    }

    Ok(CollectRewardsQuote { rewards: reward_quotes })
}
