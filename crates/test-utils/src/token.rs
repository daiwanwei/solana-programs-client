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

use crate::account::check_account_exists;

pub fn create_mint(
    svm: &mut LiteSVM,
    payer: &Keypair,
    mint_authority: &Pubkey,
    decimals: u8,
) -> Result<(Pubkey, TransactionMetadata)> {
    let mint = Keypair::new();
    let token_program_id = spl_token::ID;
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

pub fn mint_to(
    svm: &mut LiteSVM,
    authority: &Pubkey,
    mint: &Pubkey,
    destination: &Pubkey,
    signers: Option<&[&Keypair]>,
    amount: u64,
) -> Result<TransactionMetadata> {
    let signers = signers.unwrap_or(&[]);
    let signer_pubkeys = signers.iter().map(|s| s.pubkey()).collect::<Vec<_>>();

    let token_program_id = spl_token::ID;

    let mint_to_ix = token_instruction::mint_to(
        &token_program_id,
        &mint,
        &destination,
        &authority,
        &signer_pubkeys.iter().map(|s| s).collect::<Vec<_>>(),
        amount,
    )?;

    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_ix],
        Some(&authority),
        signers,
        svm.latest_blockhash(),
    );
    let result = svm.send_transaction(transaction)?;

    Ok(result)
}

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

pub fn get_or_create_ata(
    svm: &mut LiteSVM,
    payer: &Keypair,
    mint: &Pubkey,
    wallet: &Pubkey,
) -> Result<(Pubkey, TransactionMetadata)> {
    let token_program_id = spl_token::ID;
    let ata = get_associated_token_address_with_program_id(&wallet, &mint, &token_program_id);
    if check_account_exists(&svm, &ata) {
        return Ok((ata, TransactionMetadata::default()));
    }

    let (create_ata_ix, ata) = prepare_create_ata_instruction(mint, &payer.pubkey(), wallet)?;
    let transaction = Transaction::new_signed_with_payer(
        &[create_ata_ix],
        Some(&payer.pubkey()),
        &[&payer],
        svm.latest_blockhash(),
    );
    let result = svm.send_transaction(transaction)?;

    Ok((ata, result))
}

#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    #[error("Token error")]
    Token(#[from] spl_token::error::TokenError),

    #[error("LiteSVM error")]
    LiteSVM(FailedTransactionMetadata),

    #[error("Program error")]
    Program(#[from] ProgramError),
}

impl From<FailedTransactionMetadata> for TokenError {
    fn from(value: FailedTransactionMetadata) -> Self { TokenError::LiteSVM(value) }
}

pub type Result<T> = std::result::Result<T, TokenError>;
