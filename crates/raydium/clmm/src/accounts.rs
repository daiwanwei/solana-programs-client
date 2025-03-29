use solana_accounts_derive::ToAccountMetas;
use solana_program::pubkey::Pubkey;

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct CreateAmmConfig {
    /// Address to be set as protocol owner
    #[account(mut, signer = true)]
    pub owner: Pubkey,
    /// Initialize config state account to store protocol owner address and fee
    /// rates
    #[account(mut)]
    pub amm_config: Pubkey,
    /// System program
    pub system_program: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct SwapSingleV2 {
    /// The user performing the swap
    #[account(mut, signer = true)]
    pub payer: Pubkey,
    /// The factory state to read protocol fees
    #[account(mut)]
    pub amm_config: Pubkey,
    /// The program account of the pool in which the swap will be performed
    #[account(mut)]
    pub pool_state: Pubkey,
    /// The user token account for input token
    #[account(mut)]
    pub input_token_account: Pubkey,
    /// The user token account for output token
    #[account(mut)]
    pub output_token_account: Pubkey,
    /// The vault token account for input token
    #[account(mut)]
    pub input_vault: Pubkey,
    /// The vault token account for output token
    #[account(mut)]
    pub output_vault: Pubkey,
    /// The program account for the most recent oracle observation
    #[account(mut)]
    pub observation_state: Pubkey,
    /// SPL program for token transfers
    pub token_program: Pubkey,
    /// SPL program 2022 for token transfers
    pub token_program_2022: Pubkey,
    /// Memo program account
    pub memo_program: Pubkey,
    /// The mint of input vault token
    pub input_vault_mint: Pubkey,
    /// The mint of output vault token
    pub output_vault_mint: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct OpenPositionV2 {
    /// Pays to mint the position
    #[account(mut, signer = true)]
    pub payer: Pubkey,
    /// Receives the position NFT
    pub position_nft_owner: Pubkey,
    /// Unique token mint address
    #[account(mut, signer = true)]
    pub position_nft_mint: Pubkey,
    /// Token account where position NFT will be minted
    #[account(mut)]
    pub position_nft_account: Pubkey,
    /// To store metaplex metadata
    #[account(mut)]
    pub metadata_account: Pubkey,
    /// Add liquidity for this pool
    #[account(mut)]
    pub pool_state: Pubkey,
    /// Store the information of market marking in range
    #[account(mut)]
    pub protocol_position: Pubkey,
    /// Account to store data for the position's lower tick
    #[account(mut)]
    pub tick_array_lower: Pubkey,
    /// Account to store data for the position's upper tick
    #[account(mut)]
    pub tick_array_upper: Pubkey,
    /// personal position state
    #[account(mut)]
    pub personal_position: Pubkey,
    /// The token_0 account deposit token to the pool
    #[account(mut)]
    pub token_account_0: Pubkey,
    /// The token_1 account deposit token to the pool
    #[account(mut)]
    pub token_account_1: Pubkey,
    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_vault_0: Pubkey,
    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_vault_1: Pubkey,
    /// Sysvar for token mint and ATA creation
    pub rent: Pubkey,
    /// Program to create the position manager state account
    pub system_program: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_program: Pubkey,
    /// Program to create an ATA for receiving position NFT
    pub associated_token_program: Pubkey,
    /// Program to create NFT metadata
    pub metadata_program: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_program_2022: Pubkey,
    /// The mint of token vault 0
    pub vault_0_mint: Pubkey,
    /// The mint of token vault 1
    pub vault_1_mint: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct IncreaseLiquidityV2 {
    /// Pays to mint the position
    #[account(mut, signer = true)]
    pub nft_owner: Pubkey,
    /// The token account for nft
    pub nft_account: Pubkey,
    /// The pool state account
    #[account(mut)]
    pub pool_state: Pubkey,
    /// Store the information of market marking in range
    #[account(mut)]
    pub protocol_position: Pubkey,
    /// Increase liquidity for this position
    #[account(mut)]
    pub personal_position: Pubkey,
    /// Stores init state for the lower tick
    #[account(mut)]
    pub tick_array_lower: Pubkey,
    /// Stores init state for the upper tick
    #[account(mut)]
    pub tick_array_upper: Pubkey,
    /// The payer's token account for token_0
    #[account(mut)]
    pub token_account_0: Pubkey,
    /// The token account spending token_1 to mint the position
    #[account(mut)]
    pub token_account_1: Pubkey,
    /// The address that holds pool tokens for token_0
    #[account(mut)]
    pub token_vault_0: Pubkey,
    /// The address that holds pool tokens for token_1
    #[account(mut)]
    pub token_vault_1: Pubkey,
    /// Program to create mint account and mint tokens
    pub token_program: Pubkey,
    /// Token program 2022
    pub token_program_2022: Pubkey,
    /// The mint of token vault 0
    pub vault_0_mint: Pubkey,
    /// The mint of token vault 1
    pub vault_1_mint: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct DecreaseLiquidityV2 {
    /// The position owner or delegated authority
    #[account(mut, signer = true)]
    pub nft_owner: Pubkey,
    /// The token account for the tokenized position
    pub nft_account: Pubkey,
    /// Decrease liquidity for this position
    #[account(mut)]
    pub personal_position: Pubkey,
    /// The pool state account
    #[account(mut)]
    pub pool_state: Pubkey,
    /// Store the information of market marking in range
    #[account(mut)]
    pub protocol_position: Pubkey,
    /// Token_0 vault
    #[account(mut)]
    pub token_vault_0: Pubkey,
    /// Token_1 vault
    #[account(mut)]
    pub token_vault_1: Pubkey,
    /// Stores init state for the lower tick
    #[account(mut)]
    pub tick_array_lower: Pubkey,
    /// Stores init state for the upper tick
    #[account(mut)]
    pub tick_array_upper: Pubkey,
    /// The destination token account for receive amount_0
    #[account(mut)]
    pub recipient_token_account_0: Pubkey,
    /// The destination token account for receive amount_1
    #[account(mut)]
    pub recipient_token_account_1: Pubkey,
    /// SPL program to transfer out tokens
    pub token_program: Pubkey,
    /// Token program 2022
    pub token_program_2022: Pubkey,
    /// memo program
    pub memo_program: Pubkey,
    /// The mint of token vault 0
    pub vault_0_mint: Pubkey,
    /// The mint of token vault 1
    pub vault_1_mint: Pubkey,
}
#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct CreatePool {
    /// The funder of the pool creation
    #[account(mut, signer = true)]
    pub pool_creator: Pubkey,
    /// The AMM config account
    #[account(mut)]
    pub amm_config: Pubkey,
    /// The pool state account to be created
    #[account(mut)]
    pub pool_state: Pubkey,
    /// The token mint for token 0
    pub token_mint_0: Pubkey,
    /// The token mint for token 1
    pub token_mint_1: Pubkey,
    /// The vault account for token 0
    #[account(mut)]
    pub token_vault_0: Pubkey,
    /// The vault account for token 1
    #[account(mut)]
    pub token_vault_1: Pubkey,
    /// The observation state account
    #[account(mut)]
    pub observation_state: Pubkey,
    /// The tick array bitmap account
    #[account(mut)]
    pub tick_array_bitmap: Pubkey,
    /// Token program account
    pub token_program_0: Pubkey,
    /// Token program account
    pub token_program_1: Pubkey,
    /// System program account
    pub system_program: Pubkey,
    /// Rent sysvar account
    pub rent: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct InitializeReward {
    /// The funder of reward initialization
    #[account(mut, signer = true)]
    pub reward_funder: Pubkey,
    /// The funder's token account for reward token
    #[account(mut)]
    pub funder_token_account: Pubkey,
    /// The pool state account
    #[account(mut)]
    pub pool_state: Pubkey,
    /// The reward info state account
    #[account(mut)]
    pub reward_state: Pubkey,
    /// The vault account for reward token
    #[account(mut)]
    pub reward_token_vault: Pubkey,
    /// The reward token mint
    pub reward_token_mint: Pubkey,
    /// The funder's token account for reward token
    #[account(mut)]
    pub reward_token_account: Pubkey,
    /// Token program account
    pub token_program: Pubkey,
    /// System program account
    pub system_program: Pubkey,
    /// Rent sysvar account
    pub rent: Pubkey,
}

#[derive(ToAccountMetas, Clone, Debug, Default)]
pub struct UpdateRewardInfo {
    /// The pool state account
    #[account(mut)]
    pub pool_state: Pubkey,
}
