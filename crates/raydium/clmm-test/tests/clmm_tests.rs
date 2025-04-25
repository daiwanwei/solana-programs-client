use litesvm::LiteSVM;
use program_test_utils::{
    svm::update_clock,
    token::{get_or_create_ata, mint_to},
};
use raydium_clmm::ID;
use raydium_clmm_test::{
    builder::RaydiumClmmTestBuilder,
    operations::RaydiumClmmTest,
    types::{IncreaseLiquidityV2Params, OpenPositionV2Params, SwapV2Params},
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

// Constants for test values
const INITIAL_LIQUIDITY: u128 = 1_000_000_000;
const MAX_AMOUNT: u64 = 1_000_000_000_000_000_000;
const INCREASE_LIQUIDITY: u128 = 10_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_position() -> Result<(), Box<dyn std::error::Error>> {
        let Fixture { clmm_test, mut svm, admin: _, user0, user1: _, user2: _ } = create_fixture()?;
        update_clock(&mut svm, 1, 1000);

        // Test opening a position in the middle range
        let _unused = clmm_test.open_position_v2(
            &mut svm,
            OpenPositionV2Params {
                tick_lower_index: -30,
                tick_upper_index: 30,
                liquidity: INITIAL_LIQUIDITY,
                amount0_max: MAX_AMOUNT,
                amount1_max: MAX_AMOUNT,
                user_token_account0: user0.token_account0,
                user_token_account1: user0.token_account1,
            },
            &user0.keypair,
        )?;

        Ok(())
    }

    #[test]
    fn test_increase_liquidity() -> Result<(), Box<dyn std::error::Error>> {
        let Fixture { clmm_test, mut svm, admin: _, user0, user1: _, user2: _ } = create_fixture()?;
        update_clock(&mut svm, 1, 1000);

        // First open a position
        let (position_nft_mint, ..) = clmm_test.open_position_v2(
            &mut svm,
            OpenPositionV2Params {
                tick_lower_index: -30,
                tick_upper_index: 30,
                liquidity: INITIAL_LIQUIDITY,
                amount0_max: MAX_AMOUNT,
                amount1_max: MAX_AMOUNT,
                user_token_account0: user0.token_account0,
                user_token_account1: user0.token_account1,
            },
            &user0.keypair,
        )?;

        // Then increase liquidity
        let _unused = clmm_test.increase_liquidity_v2(
            &mut svm,
            IncreaseLiquidityV2Params {
                liquidity: INCREASE_LIQUIDITY,
                amount0_max: MAX_AMOUNT,
                amount1_max: MAX_AMOUNT,
                user_token_account0: user0.token_account0,
                user_token_account1: user0.token_account1,
                position_nft_mint,
            },
            &user0.keypair,
        )?;

        Ok(())
    }

    #[test]
    fn test_swap() -> Result<(), Box<dyn std::error::Error>> {
        let Fixture { clmm_test, mut svm, admin: _, user0, user1: _, user2: _ } = create_fixture()?;
        update_clock(&mut svm, 1, 1000);
        let zero_for_one = false;
        let is_base_input = true;
        let params = SwapV2Params {
            amount: 10000,
            other_amount_threshold: 0,
            sqrt_price_limit_x64: 0,
            is_base_input,
            zero_for_one,
            user_token_account0: user0.token_account0,
            user_token_account1: user0.token_account1,
        };

        let token_account_0_before = clmm_test.get_token_account(&mut svm, user0.token_account0)?;
        let token_account_1_before = clmm_test.get_token_account(&mut svm, user0.token_account1)?;

        let swap_state = clmm_test.preview_swap_v2(&mut svm, params.clone())?;

        // Perform a swap
        let _unused = clmm_test.swap_v2(&mut svm, params, &user0.keypair)?;

        let token_account_0_after = clmm_test.get_token_account(&mut svm, user0.token_account0)?;
        let token_account_1_after = clmm_test.get_token_account(&mut svm, user0.token_account1)?;

        if zero_for_one {
            assert_eq!(
                token_account_0_before.amount - token_account_0_after.amount,
                swap_state.amount_in
            );
            assert_eq!(
                token_account_1_after.amount - token_account_1_before.amount,
                swap_state.amount_out
            );
        } else {
            assert_eq!(
                token_account_0_after.amount - token_account_0_before.amount,
                swap_state.amount_out
            );
            assert_eq!(
                token_account_1_before.amount - token_account_1_after.amount,
                swap_state.amount_in
            );
        }

        Ok(())
    }

    #[test]
    fn test_open_positions_in_different_ranges() -> Result<(), Box<dyn std::error::Error>> {
        let Fixture { clmm_test, mut svm, admin: _, user0, user1: _, user2: _ } = create_fixture()?;
        update_clock(&mut svm, 1, 1000);

        // Define different tick ranges to test
        let tick_ranges = [
            (-120, -60),
            (-60, -30),
            (0, 23),
            (1, 50),
            (-30, 30),
            (30, 60),
            (40, 80),
            (60, 120),
            (120, 180),
        ];

        let mut position_nft_mints = Vec::new();

        // Open positions in different ranges
        for (tick_lower, tick_upper) in tick_ranges {
            let (position_nft_mint, ..) = clmm_test.open_position_v2(
                &mut svm,
                OpenPositionV2Params {
                    tick_lower_index: tick_lower,
                    tick_upper_index: tick_upper,
                    liquidity: INITIAL_LIQUIDITY,
                    amount0_max: MAX_AMOUNT,
                    amount1_max: MAX_AMOUNT,
                    user_token_account0: user0.token_account0,
                    user_token_account1: user0.token_account1,
                },
                &user0.keypair,
            )?;

            position_nft_mints.push(position_nft_mint);
        }

        Ok(())
    }
}

fn create_fixture() -> Result<Fixture, Box<dyn std::error::Error>> {
    let program_id = ID;
    let mut svm = LiteSVM::new().with_sigverify(false);
    svm.add_program(program_id, include_bytes!("fixtures/raydium_clmm.so"));
    let metadata_program_id =
        solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    svm.add_program(metadata_program_id, include_bytes!("fixtures/metaplex_metadata.so"));

    let admin = Keypair::new();

    let _unused = svm.airdrop(&admin.pubkey(), 1_000_000_000).unwrap();

    let clmm_test = RaydiumClmmTestBuilder::new().build(&mut svm, &admin).unwrap();
    let user0 =
        create_user(&mut svm, &admin, &clmm_test.token_pair.mint0, &clmm_test.token_pair.mint1);
    let user1 =
        create_user(&mut svm, &admin, &clmm_test.token_pair.mint0, &clmm_test.token_pair.mint1);
    let user2 =
        create_user(&mut svm, &admin, &clmm_test.token_pair.mint0, &clmm_test.token_pair.mint1);

    // Define different tick ranges
    let tick_ranges =
        [(-120, -60), (-60, -30), (-10, 0), (0, 23), (1, 50), (-30, 30), (30, 60), (60, 120)];

    let mut position_nft_mints = Vec::new();

    // Open positions in different ranges
    for (tick_lower, tick_upper) in tick_ranges {
        let (position_nft_mint, ..) = clmm_test.open_position_v2(
            &mut svm,
            OpenPositionV2Params {
                tick_lower_index: tick_lower,
                tick_upper_index: tick_upper,
                liquidity: 1_000_000,
                amount0_max: MAX_AMOUNT,
                amount1_max: MAX_AMOUNT,
                user_token_account0: user0.token_account0,
                user_token_account1: user0.token_account1,
            },
            &user0.keypair,
        )?;
        update_clock(&mut svm, 1, 1000);

        let _unused = clmm_test.increase_liquidity_v2(
            &mut svm,
            IncreaseLiquidityV2Params {
                liquidity: 1_000_000,
                amount0_max: MAX_AMOUNT,
                amount1_max: MAX_AMOUNT,
                user_token_account0: user0.token_account0,
                user_token_account1: user0.token_account1,
                position_nft_mint,
            },
            &user0.keypair,
        )?;

        position_nft_mints.push(position_nft_mint);
    }

    Ok(Fixture { clmm_test, svm, admin, user0, user1, user2 })
}

fn create_user(svm: &mut LiteSVM, admin: &Keypair, mint_0: &Pubkey, mint_1: &Pubkey) -> User {
    let user = Keypair::new();
    let _unused = svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();
    let (token_account0, _) = get_or_create_ata(svm, admin, mint_0, &user.pubkey()).unwrap();
    let (token_account1, _) = get_or_create_ata(svm, admin, mint_1, &user.pubkey()).unwrap();

    let _unused =
        mint_to(svm, admin, mint_0, &token_account0, &[admin], 1_000_000_000_000_000_000).unwrap();
    let _unused =
        mint_to(svm, admin, mint_1, &token_account1, &[admin], 1_000_000_000_000_000_000).unwrap();

    User { keypair: user, token_account0, token_account1 }
}

#[allow(dead_code)]
struct Fixture {
    clmm_test: RaydiumClmmTest,
    svm: LiteSVM,

    admin: Keypair,

    user0: User,
    user1: User,
    user2: User,
}

struct User {
    keypair: Keypair,
    token_account0: Pubkey,
    token_account1: Pubkey,
}
