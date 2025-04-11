use litesvm::{
    types::{FailedTransactionMetadata, TransactionMetadata},
    LiteSVM,
};
use solana_sdk::{
    clock::Clock,
    instruction::Instruction,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

pub fn update_clock(svm: &mut LiteSVM, slot: u64, unix_timestamp: i64) {
    let mut clock = svm.get_sysvar::<Clock>();
    clock.slot = slot;
    clock.unix_timestamp = unix_timestamp;
    svm.set_sysvar(&clock);
}

pub fn sign_and_send_transaction(
    svm: &mut LiteSVM,
    instructions: &[Instruction],
    payer: &Keypair,
    signers: Option<&[&Keypair]>,
) -> Result<TransactionMetadata, FailedTransactionMetadata> {
    let mut transaction = Transaction::new_with_payer(instructions, Some(&payer.pubkey()));
    let last_blockhash = svm.latest_blockhash();
    transaction.partial_sign(&[payer], last_blockhash);
    if let Some(signers) = signers {
        transaction.partial_sign(signers, last_blockhash);
    }
    let metadata = svm.send_transaction(transaction)?;
    Ok(metadata)
}
