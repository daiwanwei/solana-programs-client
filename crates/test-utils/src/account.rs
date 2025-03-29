use anchor_trait::Discriminator;
use borsh::BorshDeserialize;
use litesvm::LiteSVM;
use solana_deserialize::account::{deserialize_anchor_account, deserialize_solana_account};
use solana_program_pack::Pack;
use solana_sdk::pubkey::Pubkey;

pub fn check_account_exists(svm: &LiteSVM, pubkey: &Pubkey) -> bool {
    svm.get_account(pubkey).is_some()
}

pub fn get_anchor_account<T: Discriminator + BorshDeserialize>(
    svm: &LiteSVM,
    pubkey: &Pubkey,
) -> Option<T> {
    let account = svm.get_account(pubkey)?;

    let account = deserialize_anchor_account::<T>(&account.data).ok()?;

    Some(account)
}

pub fn get_solana_account<T: Pack>(svm: &LiteSVM, pubkey: &Pubkey) -> Option<T> {
    let account = svm.get_account(pubkey)?;

    let account = deserialize_solana_account::<T>(&account.data).ok()?;

    Some(account)
}
