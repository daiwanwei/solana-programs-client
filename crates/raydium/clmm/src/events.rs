use anchor_trait::Discriminator;
use anchor_trait_derive::discriminator;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[discriminator(event)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct SwapEvent {
    /// The pool for which token_0 and token_1 were swapped
    pub pool_state: Pubkey,

    /// The address that initiated the swap call, and that received the callback
    pub sender: Pubkey,

    /// The payer token account in zero for one swaps, or the recipient token
    /// account in one for zero swaps
    pub token_account_0: Pubkey,

    /// The payer token account in one for zero swaps, or the recipient token
    /// account in zero for one swaps
    pub token_account_1: Pubkey,

    /// The real delta amount of the token_0 of the pool or user
    pub amount_0: u64,

    /// The transfer fee charged by the withheld_amount of the token_0
    pub transfer_fee_0: u64,

    /// The real delta of the token_1 of the pool or user
    pub amount_1: u64,

    /// The transfer fee charged by the withheld_amount of the token_1
    pub transfer_fee_1: u64,

    /// if true, amount_0 is negtive and amount_1 is positive
    pub zero_for_one: bool,

    /// The sqrt(price) of the pool after the swap, as a Q64.64
    pub sqrt_price_x64: u128,

    /// The liquidity of the pool after the swap
    pub liquidity: u128,

    /// The log base 1.0001 of price of the pool after the swap
    pub tick: i32,
}

#[cfg(test)]
mod tests {
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    use super::*;

    #[test]
    fn test_deserialize_swap_event() {
        let serialized = "QMbN6CYIceIpVWpkzMLdriyW1P//w/S8BkpgzdDUa8y8cOO1G2aLKAVK1aJDHzjsNrxiUZ0dow0jvgF2goaCLeMgxh2kZzNbqllDUdjaOlHL2ipn48Dp45di1uoe7Uy6JO8wDlT0vWgCM6A/0KgbUQeZISlIXXQr809EzABVyOzaJT2O6K4X98U5+FMDAAAAAAAAAAAAAABBbVGlAAAAAAAAAAAAAAAAAMq0VQY8X8VwAAAAAAAAAADIS7p5GCgAAAAAAAAAAAAA8r///w==";

        // 使用 base64 解码
        let decoded = STANDARD.decode(serialized).expect("Failed to decode base64 string");

        // 反序列化为 SwapEvent
        let swap_event =
            SwapEvent::deserialize(&mut &decoded[8..]).expect("Failed to deserialize SwapEvent");
        println!("swap_event: {:?}", swap_event);
        println!("swap_event discriminator: {:?}", SwapEvent::DISCRIMINATOR);
        println!("swap_event discriminator 2: {:?}", &decoded[0..8]);
    }
}
