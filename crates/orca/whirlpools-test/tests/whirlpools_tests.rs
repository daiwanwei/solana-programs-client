use litesvm::LiteSVM;
use orca_whirlpools::ID;
use orca_whirlpools_test::{
    fixture::{setup_user, WhirlpoolConfigFixtureBuilder, WhirlpoolFixtureBuilder},
    tester::WhirlpoolsTester,
    types::{
        CreateFeeTierParams, CreateWhirlPoolTesterParams, IncreaseLiquidityParams,
        OpenPositionParams, SwapParams, User, WhirlpoolConfigFixture, WhirlpoolFixture,
    },
};
use program_test_utils::svm::update_clock;
use solana_sdk::signature::{Keypair, Signer};

// Constants for test values
const MAX_AMOUNT: u64 = 1_000_000_000_000_000_000;
const INCREASE_LIQUIDITY: u128 = 10_000;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_position() -> Result<(), Box<dyn std::error::Error>> {
        let Fixture { whirlpools_tester, mut svm, user0, .. } = create_fixture()?;
        update_clock(&mut svm, 1, 1000);

        // Test opening a position in the middle range
        let _unused = whirlpools_tester.open_position(
            &mut svm,
            &user0.keypair,
            OpenPositionParams {
                owner: user0.keypair.pubkey(),
                tick_lower_index: -30,
                tick_upper_index: 30,
            },
        )?;

        Ok(())
    }

    #[test]
    fn test_increase_liquidity() -> Result<(), Box<dyn std::error::Error>> {
        let Fixture { whirlpools_tester, mut svm, user0, .. } = create_fixture()?;
        update_clock(&mut svm, 1, 1000);

        // First open a position
        let (position_nft_mint, ..) = whirlpools_tester.open_position(
            &mut svm,
            &user0.keypair,
            OpenPositionParams {
                owner: user0.keypair.pubkey(),
                tick_lower_index: -30,
                tick_upper_index: 30,
            },
        )?;

        // Then increase liquidity
        let _unused = whirlpools_tester.increase_liquidity(
            &mut svm,
            &user0.keypair,
            IncreaseLiquidityParams {
                nft_owner: user0.keypair.pubkey(),
                position_nft_mint,
                token_account_a: user0.token_account_0,
                token_account_b: user0.token_account_1,
                liquidity: INCREASE_LIQUIDITY,
                token_max_a: MAX_AMOUNT,
                token_max_b: MAX_AMOUNT,
            },
        )?;

        Ok(())
    }

    #[test]
    fn test_swap() -> Result<(), Box<dyn std::error::Error>> {
        let Fixture { whirlpools_tester, mut svm, user0, .. } = create_fixture()?;
        update_clock(&mut svm, 1, 1000);

        let a_to_b = false;
        let is_base_input = false;
        let specified_amount = 100;

        let (amount_in, amount_out, threshold) =
            whirlpools_tester.preview_swap(&svm, specified_amount, is_base_input, a_to_b)?;

        let params = SwapParams {
            token_authority: user0.keypair.pubkey(),
            token_owner_account_a: user0.token_account_0,
            token_owner_account_b: user0.token_account_1,
            amount: specified_amount,
            other_amount_threshold: threshold,
            sqrt_price_limit: 0,
            amount_specified_is_input: is_base_input,
            a_to_b,
        };

        let token_account_0_before =
            whirlpools_tester.get_token_account(&mut svm, &user0.token_account_0)?;
        let token_account_1_before =
            whirlpools_tester.get_token_account(&mut svm, &user0.token_account_1)?;

        // Perform a swap
        let _unused = whirlpools_tester.swap(&mut svm, &user0.keypair, params)?;

        let token_account_0_after =
            whirlpools_tester.get_token_account(&mut svm, &user0.token_account_0)?;
        let token_account_1_after =
            whirlpools_tester.get_token_account(&mut svm, &user0.token_account_1)?;

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

    let fee_tier_params = vec![CreateFeeTierParams { tick_spacing: 1, default_fee_rate: 0 }];

    let whirlpool_config_fixture = WhirlpoolConfigFixtureBuilder::new()
        .with_fee_tier_params(fee_tier_params)
        .build(&mut svm, &admin)?;

    let whirlpool_fixture = WhirlpoolFixtureBuilder::new().build(
        &mut svm,
        &admin,
        whirlpool_config_fixture.whirlpool_config,
        whirlpool_config_fixture.fee_tier_list[0].fee_tier,
        whirlpool_config_fixture.fee_tier_list[0].tick_spacing,
    )?;

    let whirlpool_fixture_clone = whirlpool_fixture.clone();

    let whirlpools_tester = WhirlpoolsTester::new(CreateWhirlPoolTesterParams {
        program_id,
        whirlpool_config: whirlpool_fixture_clone.whirlpool_config,
        fee_tier: whirlpool_fixture_clone.fee_tier,
        whirlpool: whirlpool_fixture_clone.whirlpool,
        token_pair: whirlpool_fixture_clone.token_pair,
        token_vault_a: whirlpool_fixture_clone.token_vault_a,
        token_vault_b: whirlpool_fixture_clone.token_vault_b,
        tick_spacing: whirlpool_fixture_clone.tick_spacing,
    });

    let user0 = setup_user(
        &mut svm,
        &admin,
        &whirlpool_fixture.token_pair.mint_a,
        &whirlpool_fixture.token_pair.mint_b,
    );

    let user1 = setup_user(
        &mut svm,
        &admin,
        &whirlpool_fixture.token_pair.mint_a,
        &whirlpool_fixture.token_pair.mint_b,
    );

    let user2 = setup_user(
        &mut svm,
        &admin,
        &whirlpool_fixture.token_pair.mint_a,
        &whirlpool_fixture.token_pair.mint_b,
    );

    Ok(Fixture {
        whirlpool_config_fixture,
        whirlpool_fixture,
        whirlpools_tester,
        svm,
        admin,
        user0,
        user1,
        user2,
    })
}

#[allow(dead_code)]
struct Fixture {
    whirlpool_config_fixture: WhirlpoolConfigFixture,
    whirlpool_fixture: WhirlpoolFixture,
    whirlpools_tester: WhirlpoolsTester,
    svm: LiteSVM,
    admin: Keypair,
    user0: User,
    user1: User,
    user2: User,
}
