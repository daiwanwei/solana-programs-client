use borsh::BorshDeserialize;
use solana_program_pack::Pack;

use crate::account::error::{AccountError, Result};

pub fn deserialize_solana_account_by_pack<T>(data: &[u8]) -> Result<T>
where
    T: Pack,
{
    let account =
        T::unpack_unchecked(&mut &data[..]).map_err(|_| AccountError::DeserializeSolanaAccount)?;

    Ok(account)
}

pub fn deserialize_solana_account_by_borsh<T>(data: &[u8]) -> Result<T>
where
    T: BorshDeserialize,
{
    let account =
        T::deserialize(&mut &data[..]).map_err(|_| AccountError::DeserializeSolanaAccount)?;

    Ok(account)
}
