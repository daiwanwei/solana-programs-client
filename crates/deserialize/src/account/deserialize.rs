use anchor_trait::Discriminator;
use borsh::BorshDeserialize;
use snafu::ResultExt;
use solana_program_pack::Pack;

use crate::account::error::{self, AccountError, Result};

pub fn deserialize_anchor_account<T>(data: &[u8]) -> Result<T>
where
    T: BorshDeserialize + Discriminator,
{
    if data.len() < 8 {
        return Err(AccountError::InvalidDiscriminatorLength { actual: data.len() });
    }

    let discriminator = data[0..8].try_into().map_err(|_| AccountError::ParseDiscriminator)?;

    if discriminator != T::DISCRIMINATOR {
        return Err(AccountError::InvalidDiscriminator {
            expected: T::DISCRIMINATOR,
            actual: discriminator,
        });
    }

    let account = T::deserialize(&mut &data[8..]).context(error::DeserializeAnchorAccountSnafu)?;

    Ok(account)
}

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
