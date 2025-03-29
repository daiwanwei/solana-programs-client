use litesvm::{
    types::{FailedTransactionMetadata, TransactionMetadata},
    LiteSVM,
};
use solana_sdk::{
    clock::Clock, instruction::Instruction, pubkey::Pubkey, signature::Keypair,
    transaction::Transaction,
};

pub fn update_clock(svm: &mut LiteSVM, slot: u64, unix_timestamp: i64) {
    let mut clock = svm.get_sysvar::<Clock>();
    clock.slot = slot;
    clock.unix_timestamp = unix_timestamp;
    svm.set_sysvar(&clock);
}

pub fn prepare_and_send_transaction(
    svm: &mut LiteSVM,
    ixs: &[Instruction],
    payer: Option<&Pubkey>,
    signers: &[&Keypair],
) -> Result<TransactionMetadata, FailedTransactionMetadata> {
    let transaction =
        Transaction::new_signed_with_payer(ixs, payer, signers, svm.latest_blockhash());

    svm.send_transaction(transaction)
}
