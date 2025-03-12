use anchor_instructions_derive::Instructions;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Instructions)]
pub enum ClmmInstruction {
    InitializePool {
        bump: u8,
    },
    Swap {
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
    },
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
