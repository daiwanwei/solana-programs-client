use litesvm::LiteSVM;
use orca_whirlpools::ID;
use orca_whirlpools_test::{
    builder::WhirlpoolsTestBuilder,
    operations::WhirlpoolsTest,
    types::{IncreaseLiquidityParams, OpenPositionParams, SwapParams},
};
use program_test_utils::{
    svm::update_clock,
    token::{get_or_create_ata, mint_to},
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

// Constants for test values
const MAX_AMOUNT: u64 = 1_000_000_000_000_000_000;
const INCREASE_LIQUIDITY: u128 = 10_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_position() -> Result<(), Box<dyn std::error::Error>> {
        let Fixture { whirlpools_test, mut svm, admin: _, user0, user1: _, user2: _ } =
            create_fixture()?;
        update_clock(&mut svm, 1, 1000);

        // Test opening a position in the middle range
        let _unused = whirlpools_test.open_position(
            &mut svm,
            &user0.keypair,
            OpenPositionParams {
                owner: user0.keypair.pubkey(),
                whirlpool: whirlpools_test.whirlpool,
                tick_lower_index: -30,
                tick_upper_index: 30,
            },
        )?;

        Ok(())
    }

    #[test]
    fn test_increase_liquidity() -> Result<(), Box<dyn std::error::Error>> {
        let Fixture { whirlpools_test, mut svm, admin: _, user0, user1: _, user2: _ } =
            create_fixture()?;
        update_clock(&mut svm, 1, 1000);

        // First open a position
        let (position_nft_mint, ..) = whirlpools_test.open_position(
            &mut svm,
            &user0.keypair,
            OpenPositionParams {
                owner: user0.keypair.pubkey(),
                whirlpool: whirlpools_test.whirlpool,
                tick_lower_index: -30,
                tick_upper_index: 30,
            },
        )?;

        // Then increase liquidity
        let _unused = whirlpools_test.increase_liquidity(
            &mut svm,
            &user0.keypair,
            IncreaseLiquidityParams {
                nft_owner: user0.keypair.pubkey(),
                whirlpool: whirlpools_test.whirlpool,
                position_nft_mint,
                token_account_a: user0.token_account_0,
                token_account_b: user0.token_account_1,
                token_vault_a: whirlpools_test.token_vault_a,
                token_vault_b: whirlpools_test.token_vault_b,
                liquidity: INCREASE_LIQUIDITY,
                token_max_a: MAX_AMOUNT,
                token_max_b: MAX_AMOUNT,
            },
        )?;

        Ok(())
    }

    #[test]
    fn test_swap() -> Result<(), Box<dyn std::error::Error>> {
        let Fixture { whirlpools_test, mut svm, admin: _, user0, user1: _, user2: _ } =
            create_fixture()?;
        update_clock(&mut svm, 1, 1000);

        let a_to_b = false;
        let is_base_input = false;
        let specified_amount = 100;

        let (amount_in, amount_out, threshold) =
            whirlpools_test.preview_swap(&svm, specified_amount, is_base_input, a_to_b)?;

        let params = SwapParams {
            token_authority: user0.keypair.pubkey(),
            whirlpool: whirlpools_test.whirlpool,
            token_owner_account_a: user0.token_account_0,
            token_vault_a: whirlpools_test.token_vault_a,
            token_owner_account_b: user0.token_account_1,
            token_vault_b: whirlpools_test.token_vault_b,
            amount: specified_amount,
            other_amount_threshold: threshold,
            sqrt_price_limit: 0,
            amount_specified_is_input: is_base_input,
            a_to_b,
        };

        let token_account_0_before =
            whirlpools_test.get_token_account(&mut svm, &user0.token_account_0)?;
        let token_account_1_before =
            whirlpools_test.get_token_account(&mut svm, &user0.token_account_1)?;

        // Perform a swap
        let _unused = whirlpools_test.swap(&mut svm, &user0.keypair, params)?;

        let token_account_0_after =
            whirlpools_test.get_token_account(&mut svm, &user0.token_account_0)?;
        let token_account_1_after =
            whirlpools_test.get_token_account(&mut svm, &user0.token_account_1)?;

        if a_to_b {
            let amount_in_value = token_account_0_before.amount - token_account_0_after.amount;
            let amount_out_value = token_account_1_after.amount - token_account_1_before.amount;
            assert_eq!(amount_in, amount_in_value);
            assert_eq!(amount_out, amount_out_value);
        } else {
            let amount_in_value = token_account_1_before.amount - token_account_1_after.amount;
            let amount_out_value = token_account_0_after.amount - token_account_0_before.amount;
            assert_eq!(amount_in, amount_in_value);
            assert_eq!(amount_out, amount_out_value);
        }

        Ok(())
    }
}

fn create_fixture() -> Result<Fixture, Box<dyn std::error::Error>> {
    let program_id = ID;
    let mut svm = LiteSVM::new().with_sigverify(false);
    svm.add_program(program_id, include_bytes!("fixtures/orca_whirlpools.so"));

    let admin = Keypair::new();
    let _unused = svm.airdrop(&admin.pubkey(), 1_000_000_000_000).unwrap();

    let whirlpools_test = WhirlpoolsTestBuilder::new().build(&mut svm, &admin)?;

    // Define different tick ranges
    let tick_ranges = [(-120, -60), (-60, -30), (0, 23), (1, 50), (-30, 30), (30, 60), (60, 120)];

    let mut position_nft_mints = Vec::new();

    let user0 = create_user(
        &mut svm,
        &admin,
        &whirlpools_test.token_pair.mint_a,
        &whirlpools_test.token_pair.mint_b,
    );

    let user1 = create_user(
        &mut svm,
        &admin,
        &whirlpools_test.token_pair.mint_a,
        &whirlpools_test.token_pair.mint_b,
    );

    let user2 = create_user(
        &mut svm,
        &admin,
        &whirlpools_test.token_pair.mint_a,
        &whirlpools_test.token_pair.mint_b,
    );

    // Open positions in different ranges
    for (tick_lower, tick_upper) in tick_ranges {
        let (position_nft_mint, ..) = whirlpools_test.open_position(
            &mut svm,
            &user0.keypair,
            OpenPositionParams {
                owner: user0.keypair.pubkey(),
                whirlpool: whirlpools_test.whirlpool,
                tick_lower_index: tick_lower,
                tick_upper_index: tick_upper,
            },
        )?;

        update_clock(&mut svm, 1, 1000);

        let _unused = whirlpools_test.increase_liquidity(
            &mut svm,
            &user0.keypair,
            IncreaseLiquidityParams {
                nft_owner: user0.keypair.pubkey(),
                whirlpool: whirlpools_test.whirlpool,
                position_nft_mint,
                token_account_a: user0.token_account_0,
                token_account_b: user0.token_account_1,
                token_vault_a: whirlpools_test.token_vault_a,
                token_vault_b: whirlpools_test.token_vault_b,
                liquidity: 1_000_000_000,
                token_max_a: MAX_AMOUNT,
                token_max_b: MAX_AMOUNT,
            },
        )?;

        position_nft_mints.push(position_nft_mint);
    }

    Ok(Fixture { whirlpools_test, svm, admin, user0, user1, user2 })
}

fn create_user(svm: &mut LiteSVM, admin: &Keypair, mint_a: &Pubkey, mint_b: &Pubkey) -> User {
    let user = Keypair::new();
    let _unused = svm.airdrop(&user.pubkey(), 1_000_000_000).unwrap();
    let (token_account_0, _) = get_or_create_ata(svm, admin, mint_a, &user.pubkey()).unwrap();
    let (token_account_1, _) = get_or_create_ata(svm, admin, mint_b, &user.pubkey()).unwrap();

    let _unused =
        mint_to(svm, admin, mint_a, &token_account_0, &[admin], 1_000_000_000_000_000_000).unwrap();
    let _unused =
        mint_to(svm, admin, mint_b, &token_account_1, &[admin], 1_000_000_000_000_000_000).unwrap();

    User { keypair: user, token_account_0, token_account_1 }
}

#[allow(dead_code)]
struct Fixture {
    whirlpools_test: WhirlpoolsTest,
    svm: LiteSVM,
    admin: Keypair,
    user0: User,
    user1: User,
    user2: User,
}

struct User {
    keypair: Keypair,
    token_account_0: Pubkey,
    token_account_1: Pubkey,
}
