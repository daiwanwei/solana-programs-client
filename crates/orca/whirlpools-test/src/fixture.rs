use litesvm::LiteSVM;
use orca_whirlpools::{utils::derive, ID as WHIRLPOOLS_PROGRAM_ID};
use program_test_utils::{
    sign_and_send_transaction,
    token::{get_or_create_ata, mint_to},
};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

use crate::{error::Result, types::*, utils::*};

// Constants for test values
const MAX_AMOUNT: u64 = 1_000_000_000_000_000_000;

pub struct WhirlpoolConfigFixtureBuilder {
    create_config_params: Option<CreateConfigParams>,
    create_fee_tier_params: Option<Vec<CreateFeeTierParams>>,
}

impl WhirlpoolConfigFixtureBuilder {
    pub fn new() -> Self { Self { create_config_params: None, create_fee_tier_params: None } }

    pub fn with_config_params(mut self, params: CreateConfigParams) -> Self {
        self.create_config_params = Some(params);
        self
    }

    pub fn with_fee_tier_params(mut self, params: Vec<CreateFeeTierParams>) -> Self {
        self.create_fee_tier_params = Some(params);
        self
    }

    pub fn build(self, svm: &mut LiteSVM, signer: &Keypair) -> Result<WhirlpoolConfigFixture> {
        let program_id = WHIRLPOOLS_PROGRAM_ID;

        let config_params = self.create_config_params.unwrap_or_default();
        let (config, _) = create_config(svm, &signer, program_id, config_params.clone())?;

        let mut fee_tier_list = vec![];

        for fee_tier_params in self.create_fee_tier_params.unwrap_or_default() {
            let (fee_tier, _) = initialize_fee_tier(
                svm,
                &signer,
                program_id,
                InitializeFeeTierParams {
                    whirlpool_config: config,
                    fee_authority: config_params.fee_authority,
                    tick_spacing: fee_tier_params.tick_spacing,
                    default_fee_rate: fee_tier_params.default_fee_rate,
                },
            )?;

            fee_tier_list.push(FeeTierInfo {
                fee_tier,
                tick_spacing: fee_tier_params.tick_spacing,
                default_fee_rate: fee_tier_params.default_fee_rate,
            });
        }

        Ok(WhirlpoolConfigFixture { whirlpool_config: config, fee_tier_list })
    }
}

pub struct WhirlpoolFixtureBuilder {
    mint_a: Option<Pubkey>,
    mint_b: Option<Pubkey>,
    create_mints_params: Option<CreateMintsParams>,
    create_pool_params: Option<CreatePoolParams>,
}

impl WhirlpoolFixtureBuilder {
    pub fn new() -> Self {
        Self { mint_a: None, mint_b: None, create_mints_params: None, create_pool_params: None }
    }

    pub fn with_mints(mut self, mint_a: Pubkey, mint_b: Pubkey) -> Self {
        self.mint_a = Some(mint_a);
        self.mint_b = Some(mint_b);
        self
    }

    pub fn with_mints_params(mut self, params: CreateMintsParams) -> Self {
        self.create_mints_params = Some(params);
        self
    }

    pub fn with_pool_params(mut self, params: CreatePoolParams) -> Self {
        self.create_pool_params = Some(params);
        self
    }

    pub fn build(
        self,
        svm: &mut LiteSVM,
        signer: &Keypair,
        whirlpool_config: Pubkey,
        fee_tier: Pubkey,
        tick_spacing: u16,
    ) -> Result<WhirlpoolFixture> {
        let program_id = WHIRLPOOLS_PROGRAM_ID;

        let (mint_a, mint_b, decimals_a, decimals_b, token_program_id_a, token_program_id_b) =
            if self.mint_a.is_some() && self.mint_b.is_some() {
                get_mints(svm, self.mint_a.unwrap(), self.mint_b.unwrap())?
            } else {
                create_mints(svm, &signer, self.create_mints_params.unwrap_or_default())?
            };

        let pool_params = self.create_pool_params.unwrap_or(CreatePoolParams {
            sqrt_price: orca_whirlpools_client::math::price_to_sqrt_price(
                1.0, decimals_a, decimals_b,
            ),
        });

        let (whirlpool, token_vault_a, token_vault_b) = setup_pool(
            svm,
            &signer,
            InitializePoolParams {
                whirlpool_config,
                fee_tier,
                mint_a,
                mint_b,
                tick_spacing,
                sqrt_price: pool_params.sqrt_price,
            },
        )?;

        Ok(WhirlpoolFixture {
            program_id,
            token_pair: TokenPair {
                mint_a,
                mint_b,
                decimals_a,
                decimals_b,
                token_program_id_a,
                token_program_id_b,
            },
            whirlpool_config,
            whirlpool,
            token_vault_a,
            token_vault_b,
            fee_tier,
            tick_spacing,
        })
    }
}

pub fn setup_pool(
    svm: &mut LiteSVM,
    signer: &Keypair,
    params: InitializePoolParams,
) -> Result<(Pubkey, Pubkey, Pubkey)> {
    let program_id = WHIRLPOOLS_PROGRAM_ID;

    let (whirlpool, token_vault_a, token_vault_b, _) =
        initialize_pool(svm, &signer, program_id, params.clone())?;

    let current_tick = orca_whirlpools_client::math::sqrt_price_to_tick_index(params.sqrt_price);

    let tick_start_index = orca_whirlpools_client::math::get_tick_array_start_tick_index(
        current_tick,
        params.tick_spacing,
    );

    let tick_start_index_2 =
        tick_start_index + params.tick_spacing as i32 * orca_whirlpools::constants::TICK_ARRAY_SIZE;

    setup_tick_arrays(
        svm,
        &signer,
        SetupTickArraysParams {
            whirlpool,
            start_tick: current_tick,
            tick_spacing: params.tick_spacing,
            a_to_b: true,
            array_count: 8,
        },
    )?;

    setup_tick_arrays(
        svm,
        &signer,
        SetupTickArraysParams {
            whirlpool,
            start_tick: tick_start_index_2,
            tick_spacing: params.tick_spacing,
            a_to_b: false,
            array_count: 8,
        },
    )?;

    let (token_account_0, _) =
        get_or_create_ata(svm, signer, &params.mint_a, &signer.pubkey()).unwrap();
    let (token_account_1, _) =
        get_or_create_ata(svm, signer, &params.mint_b, &signer.pubkey()).unwrap();

    let _unused = mint_to(
        svm,
        signer,
        &params.mint_a,
        &token_account_0,
        &[signer],
        1_000_000_000_000_000_000,
    )
    .unwrap();
    let _unused = mint_to(
        svm,
        signer,
        &params.mint_b,
        &token_account_1,
        &[signer],
        1_000_000_000_000_000_000,
    )
    .unwrap();

    setup_liquidity(
        svm,
        signer,
        whirlpool,
        params.mint_a,
        params.mint_b,
        token_vault_a,
        token_vault_b,
        params.tick_spacing,
    )?;

    Ok((whirlpool, token_vault_a, token_vault_b))
}

pub fn setup_liquidity(
    svm: &mut LiteSVM,
    signer: &Keypair,
    whirlpool: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    token_vault_a: Pubkey,
    token_vault_b: Pubkey,
    tick_spacing: u16,
) -> Result<()> {
    let program_id = WHIRLPOOLS_PROGRAM_ID;

    // Calculate tick ranges based on tick spacing
    let tick_ranges = [
        (-4 * tick_spacing as i32, -2 * tick_spacing as i32), // Negative range far from zero
        (-2 * tick_spacing as i32, -1 * tick_spacing as i32), // Negative range near zero
        (0, 1 * tick_spacing as i32),                         // Range starting at zero
        (1 * tick_spacing as i32, 2 * tick_spacing as i32),   // Small positive range
        (-1 * tick_spacing as i32, 1 * tick_spacing as i32),  // Range around zero
        (2 * tick_spacing as i32, 3 * tick_spacing as i32),   // Medium positive range
        (3 * tick_spacing as i32, 4 * tick_spacing as i32),   // Far positive range
    ];

    let mut position_nft_mints = Vec::new();

    let (token_account_a, _) = get_or_create_ata(svm, signer, &mint_a, &signer.pubkey()).unwrap();
    let (token_account_b, _) = get_or_create_ata(svm, signer, &mint_b, &signer.pubkey()).unwrap();

    // Open positions in different ranges
    for (tick_lower, tick_upper) in tick_ranges {
        let (open_position_ix, position_nft_mint) =
            orca_whirlpools_client::instructions::prepare_open_position_instruction(
                orca_whirlpools_client::types::OpenPositionParams {
                    payer: signer.pubkey(),
                    owner: signer.pubkey(),
                    whirlpool,
                    tick_lower_index: tick_lower,
                    tick_upper_index: tick_upper,
                },
                program_id,
            )?;

        let tick_array_lower = derive::derive_tick_array_pubkey(
            whirlpool,
            orca_whirlpools::math::tick::get_array_start_index(tick_lower, tick_spacing),
            Some(program_id),
        )
        .0;

        let tick_array_upper = derive::derive_tick_array_pubkey(
            whirlpool,
            orca_whirlpools::math::tick::get_array_start_index(tick_upper, tick_spacing),
            Some(program_id),
        )
        .0;

        let increase_liquidity_ix =
            orca_whirlpools_client::instructions::prepare_increase_liquidity_instruction(
                orca_whirlpools_client::types::IncreaseLiquidityParams {
                    nft_owner: signer.pubkey(),
                    position_nft_mint: position_nft_mint.pubkey(),
                    whirlpool,
                    token_account_a,
                    token_account_b,
                    liquidity: 1_000_000_000,
                    token_max_a: MAX_AMOUNT,
                    token_max_b: MAX_AMOUNT,
                    tick_array_lower,
                    tick_array_upper,
                    token_vault_a,
                    token_vault_b,
                    mint_a,
                    mint_b,
                },
                program_id,
            )?;

        let _unused = sign_and_send_transaction!(
            svm,
            &[open_position_ix, increase_liquidity_ix],
            signer,
            &[&position_nft_mint]
        )?;

        position_nft_mints.push(position_nft_mint);
    }

    Ok(())
}

pub fn setup_tick_arrays(
    svm: &mut LiteSVM,
    signer: &Keypair,
    params: SetupTickArraysParams,
) -> Result<()> {
    let program_id = WHIRLPOOLS_PROGRAM_ID;

    let tick_start_index = orca_whirlpools_client::math::get_tick_array_start_tick_index(
        params.start_tick,
        params.tick_spacing,
    );

    let tick_arrays_params = InitializeTickArraysParams {
        whirlpool: params.whirlpool,
        start_tick_index: tick_start_index,
        array_count: params.array_count,
        tick_spacing: params.tick_spacing,
        a_to_b: params.a_to_b,
    };

    let _unused = initialize_tick_arrays(svm, &signer, program_id, tick_arrays_params).unwrap();

    Ok(())
}

pub fn setup_user(svm: &mut LiteSVM, admin: &Keypair, mint_a: &Pubkey, mint_b: &Pubkey) -> User {
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
