use litesvm::{
    types::{FailedTransactionMetadata, TransactionMetadata},
    LiteSVM,
};
use solana_program_error::ProgramError;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction as associated_token_instruction,
};
use spl_token::instruction as token_instruction;

use crate::{account::check_account_exists, sign_and_send_transaction};

/// Creates a new token mint with the specified parameters.
///
/// # Arguments
/// * `svm` - The LiteSVM instance
/// * `payer` - The keypair that will pay for the transaction
/// * `mint_authority` - The public key that will have minting authority
/// * `decimals` - The number of decimals for the token
/// * `token_program_id` - The program ID of the token program
///
/// # Returns
/// A tuple containing the mint's public key and transaction metadata
///
/// # Errors
/// Returns an error if the transaction fails or if the token program ID is
/// invalid
pub fn create_mint(
    svm: &mut LiteSVM,
    payer: &Keypair,
    mint_authority: &Pubkey,
    decimals: u8,
    token_program_id: Pubkey,
) -> Result<(Pubkey, TransactionMetadata)> {
    if decimals > 9 {
        return Err(TokenError::InvalidDecimals(decimals));
    }

    let mint = Keypair::new();
    let space = 82;
    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        svm.minimum_balance_for_rent_exemption(space),
        space as u64,
        &token_program_id,
    );
    let create_mint_ix = token_instruction::initialize_mint(
        &token_program_id,
        &mint.pubkey(),
        &mint_authority,
        None,
        decimals,
    )?;
    let transaction = Transaction::new_signed_with_payer(
        &[create_account_ix, create_mint_ix],
        Some(&payer.pubkey()),
        &[&payer, &mint],
        svm.latest_blockhash(),
    );
    let result = svm.send_transaction(transaction)?;

    Ok((mint.pubkey(), result))
}

/// Mints tokens to a specified destination account.
///
/// # Arguments
/// * `svm` - The LiteSVM instance
/// * `authority` - The keypair with minting authority
/// * `mint` - The mint's public key
/// * `destination` - The destination token account
/// * `signers` - Additional signers required for the transaction
/// * `amount` - The amount of tokens to mint
///
/// # Returns
/// Transaction metadata on success
///
/// # Errors
/// Returns an error if the transaction fails or if the mint account doesn't
/// exist
pub fn mint_to(
    svm: &mut LiteSVM,
    authority: &Keypair,
    mint: &Pubkey,
    destination: &Pubkey,
    signers: &[&Keypair],
    amount: u64,
) -> Result<TransactionMetadata> {
    if amount == 0 {
        return Err(TokenError::InvalidAmount(amount));
    }

    let signer_pubkeys = signers.iter().map(|s| s.pubkey()).collect::<Vec<_>>();

    let mint_account = svm.get_account(mint).ok_or_else(|| TokenError::MintNotFound(*mint))?;
    let token_program_id = mint_account.owner;

    let mint_to_ix = token_instruction::mint_to(
        &token_program_id,
        &mint,
        &destination,
        &authority.pubkey(),
        &signer_pubkeys.iter().map(|s| s).collect::<Vec<_>>(),
        amount,
    )?;

    let result = sign_and_send_transaction!(svm, &[mint_to_ix], authority, signers)?;

    Ok(result)
}

/// Prepares an instruction to create an associated token account.
///
/// # Arguments
/// * `mint` - The mint's public key
/// * `payer` - The payer's public key
/// * `wallet` - The wallet's public key
///
/// # Returns
/// A tuple containing the instruction and the associated token account address
pub fn prepare_create_ata_instruction(
    mint: &Pubkey,
    payer: &Pubkey,
    wallet: &Pubkey,
) -> Result<(Instruction, Pubkey)> {
    let token_program_id = spl_token::ID;

    let create_ata_ix = associated_token_instruction::create_associated_token_account(
        &payer,
        &wallet,
        &mint,
        &token_program_id,
    );

    let ata = get_associated_token_address_with_program_id(&wallet, &mint, &token_program_id);

    Ok((create_ata_ix, ata))
}

/// Gets an existing associated token account or creates a new one.
///
/// # Arguments
/// * `svm` - The LiteSVM instance
/// * `payer` - The keypair that will pay for the transaction
/// * `mint` - The mint's public key
/// * `wallet` - The wallet's public key
///
/// # Returns
/// A tuple containing the associated token account address and transaction
/// metadata
///
/// # Errors
/// Returns an error if the transaction fails or if the mint account doesn't
/// exist
pub fn get_or_create_ata(
    svm: &mut LiteSVM,
    payer: &Keypair,
    mint: &Pubkey,
    wallet: &Pubkey,
) -> Result<(Pubkey, TransactionMetadata)> {
    let mint_account = svm.get_account(mint).ok_or_else(|| TokenError::MintNotFound(*mint))?;
    let token_program_id = mint_account.owner;

    let ata = get_associated_token_address_with_program_id(&wallet, &mint, &token_program_id);
    if check_account_exists(&svm, &ata) {
        return Ok((ata, TransactionMetadata::default()));
    }

    let (create_ata_ix, ata) = prepare_create_ata_instruction(mint, &payer.pubkey(), wallet)?;
    let result = sign_and_send_transaction!(svm, &[create_ata_ix], &payer)?;

    Ok((ata, result))
}

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("Token error: {0}")]
    Token(#[from] spl_token::error::TokenError),

    #[error("LiteSVM transaction failed")]
    LiteSVM(FailedTransactionMetadata),

    #[error("Program error: {0}")]
    Program(#[from] ProgramError),

    #[error("Invalid decimals: {0}. Must be between 0 and 9")]
    InvalidDecimals(u8),

    #[error("Invalid amount: {0}. Must be greater than 0")]
    InvalidAmount(u64),

    #[error("Mint account not found: {0}")]
    MintNotFound(Pubkey),
}

impl From<FailedTransactionMetadata> for TokenError {
    fn from(value: FailedTransactionMetadata) -> Self { TokenError::LiteSVM(value) }
}

pub type Result<T> = std::result::Result<T, TokenError>;
