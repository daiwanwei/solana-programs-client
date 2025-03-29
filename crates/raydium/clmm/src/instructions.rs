use anchor_instructions_derive::Instructions;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Instructions)]
pub enum ClmmInstruction {
    CreateAmmConfig {
        index: u16,
        tick_spacing: u16,
        trade_fee_rate: u32,
        protocol_fee_rate: u32,
        fund_fee_rate: u32,
    },
    CreatePool {
        sqrt_price_x64: u128,
        open_time: u64,
    },
    Swap {
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
    },
    #[instruction(rename = "swap_v2")]
    SwapV2 {
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
    },
    SwapRouterBaseIn {
        amount_in: u64,
        amount_out_minimum: u64,
    },

    IncreaseLiquidity {
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
    },
    #[instruction(rename = "increase_liquidity_v2")]
    IncreaseLiquidityV2 {
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
        base_flag: Option<bool>,
    },
    DecreaseLiquidity {
        liquidity: u128,
        amount_0_min: u64,
        amount_1_min: u64,
    },
    #[instruction(rename = "decrease_liquidity_v2")]
    DecreaseLiquidityV2 {
        liquidity: u128,
        amount_0_min: u64,
        amount_1_min: u64,
    },

    OpenPosition {
        tick_lower_index: i32,
        tick_upper_index: i32,
        tick_array_lower_start_index: i32,
        tick_array_upper_start_index: i32,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
    },
    #[instruction(rename = "open_position_v2")]
    OpenPositionV2 {
        tick_lower_index: i32,
        tick_upper_index: i32,
        tick_array_lower_start_index: i32,
        tick_array_upper_start_index: i32,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
        with_metadata: bool,
        base_flag: Option<bool>,
    },
    #[instruction(rename = "open_position_with_token22_nft")]
    OpenPositionWithToken22Nft {
        tick_lower_index: i32,
        tick_upper_index: i32,
        tick_array_lower_start_index: i32,
        tick_array_upper_start_index: i32,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
        with_metadata: bool,
        base_flag: Option<bool>,
    },
    ClosePosition,

    CollectProtocolFee {
        amount_0_requested: u64,
        amount_1_requested: u64,
    },
    CollectFundFee {
        amount_0_requested: u64,
        amount_1_requested: u64,
    },

    TransferRewardOwner {
        new_owner: Pubkey,
    },
    InitializeReward {
        param: InitializeRewardParam,
    },
    CollectRemainingRewards {
        reward_index: u8,
    },
    UpdateRewardInfos,
    SetRewardParams {
        reward_index: u8,
        emissions_per_second_x64: u128,
        open_time: u64,
        end_time: u64,
    },

    CreateOperationAccount,
    UpdateOperationAccount {
        param: u8,
        keys: Vec<Pubkey>,
    },
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct InitializeRewardParam {
    pub reward_index: u8,
    pub open_time: u64,
    pub end_time: u64,
    pub emissions_per_second_x64: u128,
    pub reward_mint: Pubkey,
    pub authority: Pubkey,
}

#[cfg(test)]
mod tests {
    use anchor_trait::InstructionData;

    use super::*;

    #[test]
    fn test_deserialize() {
        let ix = SwapV2 {
            amount: 1000000000000000000,
            other_amount_threshold: 1000000000000000000,
            sqrt_price_limit_x64: 1000000000000000000,
            is_base_input: true,
        };
        let data = ix.data();
        println!("data: {:?}", data);
    }
}
