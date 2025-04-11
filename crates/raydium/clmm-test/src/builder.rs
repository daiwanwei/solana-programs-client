use litesvm::LiteSVM;
use raydium_clmm::{utils::derive, ID};
use rust_decimal::Decimal;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

use crate::{
    error::Result,
    operations::RaydiumClmmTest,
    types::{CreateAmmConfigParams, CreateMintsParams, CreatePoolParams, FeeConfig, TokenPair},
    utils::{create_amm_config, create_mints, create_pool, get_mints},
};

pub struct RaydiumClmmTestBuilder {
    program_id: Option<Pubkey>,
    mint_0: Option<Pubkey>,
    mint_1: Option<Pubkey>,
    create_mints_params: Option<CreateMintsParams>,
    create_amm_config_params: Option<CreateAmmConfigParams>,
    create_pool_params: Option<CreatePoolParams>,
}

impl RaydiumClmmTestBuilder {
    pub fn new() -> Self {
        Self {
            program_id: None,
            mint_0: None,
            mint_1: None,
            create_mints_params: None,
            create_amm_config_params: None,
            create_pool_params: None,
        }
    }

    pub fn with_program_id(mut self, program_id: Pubkey) -> Self {
        self.program_id = Some(program_id);
        self
    }

    pub fn with_mints(mut self, mint_0: Pubkey, mint_1: Pubkey) -> Self {
        self.mint_0 = Some(mint_0);
        self.mint_1 = Some(mint_1);
        self
    }

    pub fn with_mints_params(mut self, params: CreateMintsParams) -> Self {
        self.create_mints_params = Some(params);
        self
    }

    pub fn with_amm_config_params(mut self, params: CreateAmmConfigParams) -> Self {
        self.create_amm_config_params = Some(params);
        self
    }

    pub fn with_pool_params(mut self, params: CreatePoolParams) -> Self {
        self.create_pool_params = Some(params);
        self
    }

    pub fn build(self, svm: &mut LiteSVM, signer: &Keypair) -> Result<RaydiumClmmTest> {
        let program_id = self.program_id.unwrap_or(ID);

        let (mint_0, mint_1, decimals_0, decimals_1, token_program_id_0, token_program_id_1) =
            if self.mint_0.is_some() && self.mint_1.is_some() {
                get_mints(svm, self.mint_0.unwrap(), self.mint_1.unwrap())?
            } else {
                create_mints(svm, &signer, self.create_mints_params.unwrap_or_default())?
            };

        let amm_config_params = self.create_amm_config_params.unwrap_or_default();
        let (amm_config, _) =
            create_amm_config(svm, &signer, program_id, amm_config_params.clone())?;

        let pool_params = self.create_pool_params.unwrap_or(CreatePoolParams {
            sqrt_price_x64: raydium_clmm_client::math::price::calculate_sqrt_price_x64(
                Decimal::from(1),
                decimals_0,
                decimals_1,
            ),
            open_time: 0,
        });
        let (pool_state, _) =
            create_pool(svm, &signer, program_id, mint_0, mint_1, amm_config, pool_params)?;

        let observation_state = derive::derive_observation_pubkey(pool_state, Some(program_id)).0;
        let tick_array_bitmap =
            derive::derive_tick_array_bitmap_pubkey(pool_state, Some(program_id)).0;
        let token_vault_0 =
            derive::derive_pool_vault_pubkey(pool_state, mint_0, Some(program_id)).0;
        let token_vault_1 =
            derive::derive_pool_vault_pubkey(pool_state, mint_1, Some(program_id)).0;

        Ok(RaydiumClmmTest {
            program_id,
            token_pair: TokenPair {
                mint_0,
                mint_1,
                decimals_0,
                decimals_1,
                token_program_id_0,
                token_program_id_1,
            },
            amm_config,
            pool_state,
            observation_state,
            tick_array_bitmap,
            token_vault_0,
            token_vault_1,
            fee_config: FeeConfig {
                tick_spacing: amm_config_params.tick_spacing,
                trade_fee_rate: amm_config_params.trade_fee_rate,
                protocol_fee_rate: amm_config_params.protocol_fee_rate,
                fund_fee_rate: amm_config_params.fund_fee_rate,
            },
        })
    }
}
