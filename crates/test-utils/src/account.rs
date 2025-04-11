use anchor_trait::Discriminator;
use borsh::BorshDeserialize;
use litesvm::LiteSVM;
use solana_deserialize::account::{deserialize_anchor_account, deserialize_solana_account};
use solana_program_pack::Pack;
use solana_sdk::{pubkey::Pubkey, stake_history::Epoch};

pub fn check_account_exists(svm: &LiteSVM, pubkey: &Pubkey) -> bool {
    svm.get_account(pubkey).is_some()
}

pub fn get_anchor_account<T: Discriminator + BorshDeserialize>(
    svm: &LiteSVM,
    pubkey: &Pubkey,
) -> Option<DecodedAccount<T>> {
    let account = svm.get_account(pubkey)?;

    let data = deserialize_anchor_account::<T>(&account.data).ok()?;

    Some(DecodedAccount {
        lamports: account.lamports,
        owner: account.owner,
        executable: account.executable,
        rent_epoch: account.rent_epoch,
        data,
    })
}

pub fn get_solana_account<T: Pack>(svm: &LiteSVM, pubkey: &Pubkey) -> Option<DecodedAccount<T>> {
    let account = svm.get_account(pubkey)?;

    let data = deserialize_solana_account::<T>(&account.data).ok()?;

    Some(DecodedAccount {
        lamports: account.lamports,
        owner: account.owner,
        executable: account.executable,
        rent_epoch: account.rent_epoch,
        data,
    })
}

pub struct DecodedAccount<T> {
    pub lamports: u64,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: Epoch,
    pub data: T,
}
