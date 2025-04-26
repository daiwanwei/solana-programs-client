use anchor_trait::Discriminator;
use borsh::BorshDeserialize;

use crate::instruction::error::{InstructionError, Result};

pub fn deserialize_anchor_instruction<T>(data: &[u8]) -> Result<T>
where
    T: BorshDeserialize + Discriminator,
{
    if data.len() < 8 {
        return Err(InstructionError::InvalidDiscriminatorLength { actual: data.len() });
    }

    let discriminator = data[0..8].try_into().map_err(|_| InstructionError::ParseDiscriminator)?;

    if discriminator != T::DISCRIMINATOR {
        return Err(InstructionError::InvalidDiscriminator {
            expected: T::DISCRIMINATOR,
            actual: discriminator,
        });
    }

    let account = T::deserialize(&mut &data[8..])
        .map_err(|e| InstructionError::DeserializeAnchorInstruction { source: e })?;

    Ok(account)
}
