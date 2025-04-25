use borsh::BorshDeserialize;
use solana_account::Account;
use solana_client_core::DecodedAccount;
use solana_program::pubkey::Pubkey;
use solana_program_pack::Pack;

use crate::account::{
    deserialize::{deserialize_solana_account_by_borsh, deserialize_solana_account_by_pack},
    error::Result,
};

pub fn decode_solana_account_by_borsh<T>(
    address: &Pubkey,
    account: &Account,
) -> Result<DecodedAccount<T>>
where
    T: BorshDeserialize,
{
    let data = account.data.as_ref();
    let data = deserialize_solana_account_by_borsh::<T>(data)?;

    Ok(DecodedAccount { address: *address, account: account.clone(), data })
}

pub fn decode_solana_account_by_pack<T>(
    address: &Pubkey,
    account: &Account,
) -> Result<DecodedAccount<T>>
where
    T: Pack,
{
    let data = account.data.as_ref();
    let data = deserialize_solana_account_by_pack::<T>(data)?;

    Ok(DecodedAccount { address: *address, account: account.clone(), data })
}
