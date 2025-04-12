use litesvm::LiteSVM;
use orca_whirlpools::ID as WHIRLPOOLS_PROGRAM_ID;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

use crate::{error::Result, operations::WhirlpoolsTest, types::*, utils::*};

pub struct WhirlpoolsTestBuilder {
    program_id: Option<Pubkey>,
    mint_a: Option<Pubkey>,
    mint_b: Option<Pubkey>,
    create_mints_params: Option<CreateMintsParams>,
    create_config_params: Option<CreateConfigParams>,
    create_fee_tier_params: Option<CreateFeeTierParams>,
    create_pool_params: Option<CreatePoolParams>,
}

impl WhirlpoolsTestBuilder {
    pub fn new() -> Self {
        Self {
            program_id: None,
            mint_a: None,
            mint_b: None,
            create_mints_params: None,
            create_config_params: None,
            create_fee_tier_params: None,
            create_pool_params: None,
        }
    }

    pub fn with_program_id(mut self, program_id: Pubkey) -> Self {
        self.program_id = Some(program_id);
        self
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

    pub fn with_config_params(mut self, params: CreateConfigParams) -> Self {
        self.create_config_params = Some(params);
        self
    }

    pub fn with_fee_tier_params(mut self, params: CreateFeeTierParams) -> Self {
        self.create_fee_tier_params = Some(params);
        self
    }

    pub fn with_pool_params(mut self, params: CreatePoolParams) -> Self {
        self.create_pool_params = Some(params);
        self
    }

    pub fn build(self, svm: &mut LiteSVM, signer: &Keypair) -> Result<WhirlpoolsTest> {
        let program_id = self.program_id.unwrap_or(WHIRLPOOLS_PROGRAM_ID);

        let (mint_a, mint_b, decimals_a, decimals_b, token_program_id_a, token_program_id_b) =
            if self.mint_a.is_some() && self.mint_b.is_some() {
                get_mints(svm, self.mint_a.unwrap(), self.mint_b.unwrap())?
            } else {
                create_mints(svm, &signer, self.create_mints_params.unwrap_or_default())?
            };

        let config_params = self.create_config_params.unwrap_or_default();
        let (config, _) = create_config(svm, &signer, program_id, config_params.clone())?;

        let fee_tier_params = self.create_fee_tier_params.unwrap_or_default();
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

        let pool_params = self.create_pool_params.unwrap_or(CreatePoolParams {
            sqrt_price: orca_whirlpools_client::math::price_to_sqrt_price(
                1.0, decimals_a, decimals_b,
            ),
        });

        let (whirlpool, token_vault_a, token_vault_b, _) = initialize_pool(
            svm,
            &signer,
            program_id,
            InitializePoolParams {
                whirlpool_config: config,
                fee_tier,
                mint_a,
                mint_b,
                tick_spacing: fee_tier_params.tick_spacing,
                sqrt_price: pool_params.sqrt_price,
            },
        )?;

        let current_tick =
            orca_whirlpools_client::math::sqrt_price_to_tick_index(pool_params.sqrt_price);

        let tick_start_index = orca_whirlpools_client::math::get_tick_array_start_tick_index(
            current_tick,
            fee_tier_params.tick_spacing,
        );

        let tick_arrays_params = InitializeTickArraysParams {
            whirlpool,
            start_tick_index: tick_start_index,
            array_count: 8,
            tick_spacing: fee_tier_params.tick_spacing,
            a_to_b: true,
        };

        let _unused = initialize_tick_arrays(svm, &signer, program_id, tick_arrays_params).unwrap();
        let tick_start_index_2 = tick_start_index
            + fee_tier_params.tick_spacing as i32 * orca_whirlpools::constants::TICK_ARRAY_SIZE;

        let tick_arrays_params_2 = InitializeTickArraysParams {
            whirlpool,
            start_tick_index: tick_start_index_2,
            array_count: 8,
            tick_spacing: fee_tier_params.tick_spacing,
            a_to_b: false,
        };

        let _unused =
            initialize_tick_arrays(svm, &signer, program_id, tick_arrays_params_2).unwrap();

        Ok(WhirlpoolsTest {
            program_id,
            token_pair: TokenPair {
                mint_a,
                mint_b,
                decimals_a,
                decimals_b,
                token_program_id_a,
                token_program_id_b,
            },
            whirlpool_config: config,
            whirlpool,
            token_vault_a,
            token_vault_b,
            fee_tier,
            tick_spacing: fee_tier_params.tick_spacing,
        })
    }
}
