use borsh::BorshDeserialize;
use litesvm::LiteSVM;
use solana_client_core::MaybeAccount;
use solana_deserialize::account::{decode_solana_account_by_borsh, decode_solana_account_by_pack};
use solana_program_pack::Pack;
use solana_sdk::pubkey::Pubkey;

pub fn check_account_exists(svm: &LiteSVM, pubkey: &Pubkey) -> bool {
    svm.get_account(pubkey).is_some()
}

pub fn get_solana_account_by_borsh<T: BorshDeserialize>(
    svm: &LiteSVM,
    pubkey: &Pubkey,
) -> MaybeAccount<T> {
    let account = if let Some(account) = svm.get_account(pubkey) {
        account
    } else {
        return MaybeAccount::NotFound(*pubkey);
    };

    let data = decode_solana_account_by_borsh::<T>(pubkey, &account)
        .inspect_err(|e| {
            println!("Failed to decode account: {:?}", e);
        })
        .unwrap();

    MaybeAccount::Exists(data)
}

pub fn get_solana_accounts_by_borsh<T: BorshDeserialize>(
    svm: &LiteSVM,
    pubkeys: &[Pubkey],
) -> Vec<MaybeAccount<T>> {
    pubkeys.iter().map(|pubkey| get_solana_account_by_borsh::<T>(svm, pubkey)).collect()
}

pub fn get_solana_account_by_pack<T: Pack>(svm: &LiteSVM, pubkey: &Pubkey) -> MaybeAccount<T> {
    let account = if let Some(account) = svm.get_account(pubkey) {
        account
    } else {
        return MaybeAccount::NotFound(*pubkey);
    };

    let data = decode_solana_account_by_pack::<T>(pubkey, &account)
        .inspect_err(|e| {
            println!("Failed to decode account: {:?}", e);
        })
        .unwrap();

    MaybeAccount::Exists(data)
}

pub fn get_solana_accounts_by_pack<T: Pack>(
    svm: &LiteSVM,
    pubkeys: &[Pubkey],
) -> Vec<MaybeAccount<T>> {
    pubkeys.iter().map(|pubkey| get_solana_account_by_pack::<T>(svm, pubkey)).collect()
}
