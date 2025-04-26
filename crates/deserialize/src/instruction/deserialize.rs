use borsh::BorshDeserialize;

use crate::instruction::{
    discriminator::validate_discriminator,
    error::{InstructionError, Result},
};

pub fn deserialize_instruction_discriminator(
    data: &[u8],
    discriminator: [u8; 8],
) -> Result<[u8; 8]> {
    if data.len() < 8 {
        return Err(InstructionError::InvalidDiscriminatorLength { actual: data.len() });
    }

    let ix_discriminator =
        data[0..8].try_into().map_err(|_| InstructionError::ParseDiscriminator)?;

    validate_discriminator(ix_discriminator, discriminator)?;

    Ok(discriminator)
}

pub fn deserialize_instruction_args_with_discriminator<T>(
    data: &[u8],
    discriminator: [u8; 8],
) -> Result<([u8; 8], T)>
where
    T: BorshDeserialize,
{
    let _ = deserialize_instruction_discriminator(data, discriminator)?;

    let args = T::deserialize(&mut &data[8..])
        .map_err(|e| InstructionError::DeserializeInstruction { source: e })?;

    Ok((discriminator, args))
}
