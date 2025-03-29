use std::sync::{Arc, Mutex};

use litesvm::{types::TransactionMetadata, LiteSVM};
use program_test_utils::{
    account::{get_anchor_account, get_solana_account},
    prepare_anchor_ix,
    svm::prepare_and_send_transaction,
    token::{create_mint, get_or_create_ata, mint_to},
};
use raydium_clmm::{
    accounts, instructions, math::tick::get_array_start_index, state, utils::derive, ID,
};
use rust_decimal::{prelude::ToPrimitive, Decimal, MathematicalOps};
use solana_sdk::{
    instruction::AccountMeta,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program, sysvar,
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, ID as SPL_ASSOCIATED_TOKEN_ACCOUNT_ID,
};
use spl_token::ID as SPL_TOKEN_ID;
use spl_token_2022::ID as SPL_TOKEN_2022_ID;

use crate::{
    constants::{ADMIN_KEY, MEMO_PROGRAM_ID, METADATA_PROGRAM_ID},
    error::{ClmmTestError, Result},
};

fn calculate_sqrt_price_x64(price: Decimal, decimals_0: u8, decimals_1: u8) -> u128 {
    // Adjust price based on token decimals
    let adjusted_price = if decimals_0 > decimals_1 {
        price * Decimal::from(10u64.pow(decimals_0 as u32 - decimals_1 as u32))
    } else {
        price / Decimal::from(10u64.pow(decimals_1 as u32 - decimals_0 as u32))
    };

    // Calculate square root
    let price_sqrt = adjusted_price.sqrt().unwrap();

    // Convert to Q64 fixed point format
    let price_sqrt_u64 = price_sqrt * Decimal::from(1u128 << 64);
    price_sqrt_u64.to_u128().unwrap()
}

pub fn get_price_from_sqrt_price_x64(
    sqrt_price_x64: u128,
    decimals_0: u8,
    decimals_1: u8,
) -> Decimal {
    let price_sqrt = normalize_price_sqrt(sqrt_price_x64);

    let price = price_sqrt * price_sqrt;

    if decimals_0 > decimals_1 {
        price * Decimal::from(10u64.pow(decimals_0 as u32 - decimals_1 as u32))
    } else {
        price / Decimal::from(10u64.pow(decimals_1 as u32 - decimals_0 as u32))
    }
}

pub fn normalize_price_sqrt(price: u128) -> Decimal {
    Decimal::from(price) / Decimal::from(2u128.pow(64))
}

pub struct RaydiumClmmTest {
    pub svm: Arc<Mutex<LiteSVM>>,
    pub program_id: Pubkey,
    pub admin: Arc<Keypair>,
    pub mint_0: Pubkey,
    pub mint_1: Pubkey,
    pub amm_config: Pubkey,
    pub pool_state: Pubkey,
    pub observation_state: Pubkey,
    pub tick_array_bitmap: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
}

pub struct RaydiumClmmTestBuilder {
    svm: Arc<Mutex<LiteSVM>>,
    program_id: Option<Pubkey>,
    admin: Option<Arc<Keypair>>,
    mint_0: Option<Pubkey>,
    mint_1: Option<Pubkey>,
    create_mints_params: Option<CreateMintsParams>,
    create_amm_config_params: Option<CreateAmmConfigParams>,
    create_pool_params: Option<CreatePoolParams>,
}

#[derive(Clone, Copy, Debug)]
pub struct CreateMintsParams {
    pub decimals_a: u8,
    pub decimals_b: u8,
}

impl Default for CreateMintsParams {
    fn default() -> Self { Self { decimals_a: 6, decimals_b: 6 } }
}

#[derive(Clone, Copy, Debug)]
pub struct CreateAmmConfigParams {
    pub program_admin: Pubkey,
    pub config_index: u16,
    pub tick_spacing: u16,
    pub trade_fee_rate: u32,
    pub protocol_fee_rate: u32,
    pub fund_fee_rate: u32,
}

impl Default for CreateAmmConfigParams {
    fn default() -> Self {
        Self {
            program_admin: ADMIN_KEY,
            config_index: 0,
            tick_spacing: 1,
            trade_fee_rate: 0,
            protocol_fee_rate: 0,
            fund_fee_rate: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CreatePoolParams {
    pub sqrt_price_x64: u128,
    pub open_time: u64,
}

impl RaydiumClmmTestBuilder {
    pub fn new(svm: Arc<Mutex<LiteSVM>>) -> Self {
        Self {
            svm,
            program_id: None,
            admin: None,
            mint_0: None,
            mint_1: None,
            create_mints_params: None,
            create_amm_config_params: None,
            create_pool_params: None,
        }
    }

    pub fn program_id(mut self, program_id: Pubkey) -> Self {
        self.program_id = Some(program_id);
        self
    }

    pub fn admin(mut self, admin: Arc<Keypair>) -> Self {
        self.admin = Some(admin);
        self
    }

    pub fn create_mints_params(mut self, create_mints_params: CreateMintsParams) -> Self {
        self.create_mints_params = Some(create_mints_params);
        self
    }

    pub fn create_amm_config_params(
        mut self,
        create_amm_config_params: CreateAmmConfigParams,
    ) -> Self {
        self.create_amm_config_params = Some(create_amm_config_params);
        self
    }

    pub fn create_pool_params(mut self, create_pool_params: CreatePoolParams) -> Self {
        self.create_pool_params = Some(create_pool_params);
        self
    }

    pub fn mint_0(mut self, mint_0: Pubkey) -> Self {
        self.mint_0 = Some(mint_0);
        self
    }

    pub fn mint_1(mut self, mint_1: Pubkey) -> Self {
        self.mint_1 = Some(mint_1);
        self
    }

    pub fn build(self) -> Result<RaydiumClmmTest> {
        let mut svm = self.svm.lock().map_err(|_| ClmmTestError::SvmLock)?;
        let program_id = self.program_id.unwrap_or(ID);

        let admin = if let Some(admin) = self.admin {
            admin
        } else {
            let keypair = Arc::new(Keypair::new());
            let _unused = svm.airdrop(&keypair.pubkey(), 1_000_000_000)?;
            keypair
        };

        let (mint_0, mint_1, decimals_0, decimals_1) =
            if self.mint_0.is_some() && self.mint_1.is_some() {
                get_mints(&svm, self.mint_0.unwrap(), self.mint_1.unwrap())?
            } else {
                create_mints(&mut svm, &admin, self.create_mints_params.unwrap_or_default())?
            };

        let (amm_config, _) = create_amm_config(
            &mut svm,
            &admin,
            program_id,
            self.create_amm_config_params.unwrap_or_default(),
        )?;
        let (pool_state, _) = create_pool(
            &mut svm,
            &admin,
            program_id,
            mint_0,
            mint_1,
            amm_config,
            self.create_pool_params.unwrap_or(CreatePoolParams {
                sqrt_price_x64: calculate_sqrt_price_x64(Decimal::from(1), decimals_0, decimals_1),
                open_time: 0,
            }),
        )?;
        let observation_state = derive::derive_observation_pubkey(pool_state, Some(program_id)).0;
        let tick_array_bitmap =
            derive::derive_tick_array_bitmap_pubkey(pool_state, Some(program_id)).0;
        let token_vault_0 =
            derive::derive_pool_vault_pubkey(pool_state, mint_0, Some(program_id)).0;
        let token_vault_1 =
            derive::derive_pool_vault_pubkey(pool_state, mint_1, Some(program_id)).0;
        Ok(RaydiumClmmTest {
            svm: self.svm.clone(),
            program_id,
            admin,
            mint_0,
            mint_1,
            amm_config,
            pool_state,
            observation_state,
            tick_array_bitmap,
            token_vault_0,
            token_vault_1,
        })
    }
}

impl RaydiumClmmTest {
    pub fn builder(svm: Arc<Mutex<LiteSVM>>) -> RaydiumClmmTestBuilder {
        RaydiumClmmTestBuilder::new(svm)
    }

    pub fn open_position_v2(
        &self,
        tick_lower_index: i32,
        tick_upper_index: i32,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
        token_account_0: Option<Pubkey>,
        token_account_1: Option<Pubkey>,
        user: Option<&Keypair>,
    ) -> Result<(Pubkey, TransactionMetadata)> {
        let mut svm = self.svm.lock().map_err(|_| ClmmTestError::SvmLock)?;
        let payer = if let Some(payer) = user { payer } else { &self.admin };

        let pool_state = get_anchor_account::<state::PoolState>(&svm, &self.pool_state).unwrap();

        let tick_array_lower_start_index =
            get_array_start_index(tick_lower_index, pool_state.tick_spacing);
        let tick_array_upper_start_index =
            get_array_start_index(tick_upper_index, pool_state.tick_spacing);

        let open_position_v2_ix = instructions::OpenPositionV2 {
            tick_lower_index,
            tick_upper_index,
            tick_array_lower_start_index,
            tick_array_upper_start_index,
            liquidity,
            amount_0_max,
            amount_1_max,
            with_metadata: false,
            base_flag: Some(false),
        };

        let position_nft_mint = Keypair::new();

        let token_account_0 = if let Some(token_account_0) = token_account_0 {
            token_account_0
        } else {
            get_associated_token_address_with_program_id(
                &payer.pubkey(),
                &self.mint_0,
                &SPL_TOKEN_ID,
            )
        };

        let token_account_1 = if let Some(token_account_1) = token_account_1 {
            token_account_1
        } else {
            get_associated_token_address_with_program_id(
                &payer.pubkey(),
                &self.mint_1,
                &SPL_TOKEN_ID,
            )
        };

        let personal_position = derive::derive_personal_position_pubkey(
            position_nft_mint.pubkey(),
            Some(self.program_id),
        )
        .0;

        let protocol_position = derive::derive_protocol_position_pubkey(
            self.pool_state,
            tick_lower_index,
            tick_upper_index,
            Some(self.program_id),
        )
        .0;

        let tick_array_lower = derive::derive_tick_array_pubkey(
            self.pool_state,
            tick_array_lower_start_index,
            Some(self.program_id),
        )
        .0;

        let tick_array_upper = derive::derive_tick_array_pubkey(
            self.pool_state,
            tick_array_upper_start_index,
            Some(self.program_id),
        )
        .0;

        let position_nft_account = get_associated_token_address_with_program_id(
            &payer.pubkey(),
            &position_nft_mint.pubkey(),
            &SPL_TOKEN_ID,
        );

        let open_position_v2_accounts = accounts::OpenPositionV2 {
            payer: payer.pubkey(),
            pool_state: self.pool_state,
            position_nft_owner: payer.pubkey(),
            position_nft_mint: position_nft_mint.pubkey(),
            position_nft_account,
            metadata_account: Pubkey::default(),
            protocol_position,
            tick_array_lower,
            tick_array_upper,
            personal_position,
            token_account_0,
            token_account_1,
            token_vault_0: self.token_vault_0,
            token_vault_1: self.token_vault_1,
            token_program: SPL_TOKEN_ID,
            associated_token_program: SPL_ASSOCIATED_TOKEN_ACCOUNT_ID,
            metadata_program: METADATA_PROGRAM_ID,
            token_program_2022: SPL_TOKEN_2022_ID,
            vault_0_mint: self.mint_0,
            vault_1_mint: self.mint_1,
            rent: sysvar::rent::ID,
            system_program: system_program::ID,
        };

        let instruction = prepare_anchor_ix!(
            self.program_id,
            open_position_v2_ix,
            open_position_v2_accounts,
            None
        );

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer, &position_nft_mint],
            svm.latest_blockhash(),
        );

        let metadata = svm.send_transaction(transaction)?;

        Ok((position_nft_mint.pubkey(), metadata))
    }

    pub fn increase_liquidity(
        &self,
        position_nft_mint: Pubkey,
        liquidity: u128,
        amount_0_max: u64,
        amount_1_max: u64,
        token_account_0: Option<Pubkey>,
        token_account_1: Option<Pubkey>,
        user: Option<&Keypair>,
    ) -> Result<TransactionMetadata> {
        let mut svm = self.svm.lock().map_err(|_| ClmmTestError::SvmLock)?;
        let payer = if let Some(payer) = user { payer } else { &self.admin };

        let token_account_0 = if let Some(token_account_0) = token_account_0 {
            token_account_0
        } else {
            get_associated_token_address_with_program_id(
                &payer.pubkey(),
                &self.mint_0,
                &SPL_TOKEN_ID,
            )
        };

        let token_account_1 = if let Some(token_account_1) = token_account_1 {
            token_account_1
        } else {
            get_associated_token_address_with_program_id(
                &payer.pubkey(),
                &self.mint_1,
                &SPL_TOKEN_ID,
            )
        };

        let pool_state = get_anchor_account::<state::PoolState>(&svm, &self.pool_state)
            .ok_or(ClmmTestError::PoolStateNotFound)?;

        let nft_account = get_associated_token_address_with_program_id(
            &payer.pubkey(),
            &position_nft_mint,
            &SPL_TOKEN_ID,
        );

        let personal_position =
            derive::derive_personal_position_pubkey(position_nft_mint, Some(self.program_id)).0;

        let position_account =
            get_anchor_account::<state::PersonalPositionState>(&svm, &personal_position)
                .ok_or(ClmmTestError::PersonalPositionNotFound)?;

        let protocol_position = derive::derive_protocol_position_pubkey(
            self.pool_state,
            position_account.tick_lower_index,
            position_account.tick_upper_index,
            Some(self.program_id),
        )
        .0;

        let tick_lower_start_index =
            get_array_start_index(position_account.tick_lower_index, pool_state.tick_spacing);

        let tick_upper_start_index =
            get_array_start_index(position_account.tick_upper_index, pool_state.tick_spacing);

        let tick_array_lower = derive::derive_tick_array_pubkey(
            self.pool_state,
            tick_lower_start_index,
            Some(self.program_id),
        )
        .0;

        let tick_array_upper = derive::derive_tick_array_pubkey(
            self.pool_state,
            tick_upper_start_index,
            Some(self.program_id),
        )
        .0;

        let increase_liquidity_ix = instructions::IncreaseLiquidityV2 {
            liquidity,
            amount_0_max,
            amount_1_max,
            base_flag: Some(false),
        };

        let increase_liquidity_accounts = accounts::IncreaseLiquidityV2 {
            nft_owner: payer.pubkey(),
            pool_state: self.pool_state,
            nft_account,
            protocol_position,
            personal_position,
            tick_array_lower,
            tick_array_upper,
            token_account_0,
            token_account_1,
            token_vault_0: self.token_vault_0,
            token_vault_1: self.token_vault_1,
            token_program: SPL_TOKEN_ID,
            token_program_2022: SPL_TOKEN_2022_ID,
            vault_0_mint: self.mint_0,
            vault_1_mint: self.mint_1,
        };

        let instruction = prepare_anchor_ix!(
            self.program_id,
            increase_liquidity_ix,
            increase_liquidity_accounts,
            None
        );

        let metadata = prepare_and_send_transaction(
            &mut svm,
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
        )?;

        Ok(metadata)
    }

    pub fn swap_v2(
        &self,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit_x64: u128,
        is_base_input: bool,
        zero_for_one: bool,
        input_token_account: Option<Pubkey>,
        output_token_account: Option<Pubkey>,
        user: Option<&Keypair>,
    ) -> Result<TransactionMetadata> {
        let mut svm = self.svm.lock().map_err(|_| ClmmTestError::SvmLock)?;
        let payer = if let Some(payer) = user { payer } else { &self.admin };

        let (input_mint, output_mint, input_vault, output_vault) = if zero_for_one {
            (self.mint_0, self.mint_1, self.token_vault_0, self.token_vault_1)
        } else {
            (self.mint_1, self.mint_0, self.token_vault_1, self.token_vault_0)
        };

        let input_token_account = if let Some(input_token_account) = input_token_account {
            input_token_account
        } else {
            get_associated_token_address_with_program_id(
                &payer.pubkey(),
                &input_mint,
                &SPL_TOKEN_ID,
            )
        };

        let output_token_account = if let Some(output_token_account) = output_token_account {
            output_token_account
        } else {
            get_associated_token_address_with_program_id(
                &payer.pubkey(),
                &output_mint,
                &SPL_TOKEN_ID,
            )
        };

        let pool_state = get_anchor_account::<state::PoolState>(&svm, &self.pool_state)
            .ok_or(ClmmTestError::PoolStateNotFound)?;

        let swap_v2_ix = instructions::SwapV2 {
            amount,
            other_amount_threshold,
            sqrt_price_limit_x64,
            is_base_input,
        };

        let swap_v2_accounts = accounts::SwapSingleV2 {
            payer: payer.pubkey(),
            amm_config: self.amm_config,
            pool_state: self.pool_state,
            input_token_account,
            output_token_account,
            input_vault,
            output_vault,
            observation_state: self.observation_state,
            token_program: SPL_TOKEN_ID,
            token_program_2022: SPL_TOKEN_2022_ID,
            memo_program: MEMO_PROGRAM_ID,
            input_vault_mint: input_mint,
            output_vault_mint: output_mint,
        };

        let tick_start_index_0 =
            get_array_start_index(pool_state.tick_current, pool_state.tick_spacing);

        let tick_start_index_1 = if zero_for_one {
            tick_start_index_0 - 60 * pool_state.tick_spacing as i32
        } else {
            tick_start_index_0 + 60 * pool_state.tick_spacing as i32
        };

        let tick_array_0 = derive::derive_tick_array_pubkey(
            self.pool_state,
            tick_start_index_0,
            Some(self.program_id),
        )
        .0;

        let tick_array_1 = derive::derive_tick_array_pubkey(
            self.pool_state,
            tick_start_index_1,
            Some(self.program_id),
        )
        .0;

        let remaining_accounts = vec![
            AccountMeta::new(self.tick_array_bitmap, false),
            AccountMeta::new(tick_array_0, false),
            AccountMeta::new(tick_array_1, false),
        ];

        let instruction = prepare_anchor_ix!(
            self.program_id,
            swap_v2_ix,
            swap_v2_accounts,
            Some(remaining_accounts)
        );

        let metadata = prepare_and_send_transaction(
            &mut svm,
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer],
        )?;

        Ok(metadata)
    }

    pub fn mint_to(
        &self,
        mint: Pubkey,
        recipient_account: Pubkey,
        amount: u64,
    ) -> Result<TransactionMetadata> {
        let mut svm = self.svm.lock().map_err(|_| ClmmTestError::SvmLock)?;

        let metadata = mint_to(
            &mut svm,
            &self.admin.pubkey(),
            &mint,
            &recipient_account,
            Some(&[&self.admin]),
            amount,
        )?;

        Ok(metadata)
    }

    pub fn get_or_create_ata(
        &self,
        mint: Pubkey,
        wallet: Pubkey,
    ) -> Result<(Pubkey, TransactionMetadata)> {
        let mut svm = self.svm.lock().map_err(|_| ClmmTestError::SvmLock)?;
        let (ata, metadata) = get_or_create_ata(&mut svm, &self.admin, &mint, &wallet)?;

        Ok((ata, metadata))
    }
}

pub fn create_mints(
    svm: &mut LiteSVM,
    admin: &Keypair,
    params: CreateMintsParams,
) -> Result<(Pubkey, Pubkey, u8, u8)> {
    let (mint_a, _) = create_mint(svm, admin, &admin.pubkey(), params.decimals_a).unwrap();
    let (mint_b, _) = create_mint(svm, admin, &admin.pubkey(), params.decimals_b).unwrap();
    let (mint_0, mint_1, decimals_0, decimals_1) = if mint_a < mint_b {
        (mint_a, mint_b, params.decimals_a, params.decimals_b)
    } else {
        (mint_b, mint_a, params.decimals_b, params.decimals_a)
    };

    Ok((mint_0, mint_1, decimals_0, decimals_1))
}

pub fn get_mints(
    svm: &LiteSVM,
    mint_a: Pubkey,
    mint_b: Pubkey,
) -> Result<(Pubkey, Pubkey, u8, u8)> {
    let (mint_0, mint_1) = if mint_a < mint_b { (mint_a, mint_b) } else { (mint_b, mint_a) };

    let mint_0_account = get_solana_account::<spl_token::state::Mint>(svm, &mint_0)
        .ok_or(ClmmTestError::MintNotFound)?;
    let mint_1_account = get_solana_account::<spl_token::state::Mint>(svm, &mint_1)
        .ok_or(ClmmTestError::MintNotFound)?;

    Ok((mint_0, mint_1, mint_0_account.decimals, mint_1_account.decimals))
}

pub fn create_amm_config(
    svm: &mut LiteSVM,
    admin: &Keypair,
    program_id: Pubkey,
    params: CreateAmmConfigParams,
) -> Result<(Pubkey, TransactionMetadata)> {
    let create_amm_config_ix = instructions::CreateAmmConfig {
        index: params.config_index,
        tick_spacing: params.tick_spacing,
        trade_fee_rate: params.trade_fee_rate,
        protocol_fee_rate: params.protocol_fee_rate,
        fund_fee_rate: params.fund_fee_rate,
    };

    let owner = params.program_admin;
    let _unused = svm.airdrop(&owner, 1_000_000_000).unwrap();

    let amm_config = derive::derive_amm_config_pubkey(0, Some(program_id)).0;

    let create_amm_config_accounts =
        accounts::CreateAmmConfig { owner, amm_config, system_program: system_program::ID };

    let instruction =
        prepare_anchor_ix!(program_id, create_amm_config_ix, create_amm_config_accounts, None);

    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&admin.pubkey()));

    transaction.partial_sign(&[&admin], svm.latest_blockhash());
    let metadata = svm.send_transaction(transaction)?;

    Ok((amm_config, metadata))
}

pub fn create_pool(
    svm: &mut LiteSVM,
    admin: &Keypair,
    program_id: Pubkey,
    mint_0: Pubkey,
    mint_1: Pubkey,
    amm_config: Pubkey,
    params: CreatePoolParams,
) -> Result<(Pubkey, TransactionMetadata)> {
    let create_pool_ix = instructions::CreatePool {
        sqrt_price_x64: params.sqrt_price_x64,
        open_time: params.open_time,
    };

    let pool_state =
        derive::derive_pool_state_pubkey(amm_config, mint_0, mint_1, Some(program_id)).0;
    let observation_state = derive::derive_observation_pubkey(pool_state, Some(program_id)).0;
    let tick_array_bitmap = derive::derive_tick_array_bitmap_pubkey(pool_state, Some(program_id)).0;
    let token_vault_0 = derive::derive_pool_vault_pubkey(pool_state, mint_0, Some(program_id)).0;
    let token_vault_1 = derive::derive_pool_vault_pubkey(pool_state, mint_1, Some(program_id)).0;

    let create_pool_accounts = accounts::CreatePool {
        pool_creator: admin.pubkey(),
        amm_config,
        pool_state,
        token_mint_0: mint_0,
        token_mint_1: mint_1,
        token_vault_0,
        token_vault_1,
        observation_state,
        tick_array_bitmap,
        token_program_0: SPL_TOKEN_ID,
        token_program_1: SPL_TOKEN_ID,
        system_program: system_program::ID,
        rent: sysvar::rent::ID,
    };

    let instruction = prepare_anchor_ix!(program_id, create_pool_ix, create_pool_accounts, None);

    let metadata =
        prepare_and_send_transaction(svm, &[instruction], Some(&admin.pubkey()), &[&admin])?;

    Ok((pool_state, metadata))
}
