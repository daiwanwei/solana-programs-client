use crate::instruction::error::{InstructionError, Result};

pub fn get_discriminator(data: &[u8]) -> Result<[u8; 8]> {
    if data.len() < 8 {
        return Err(InstructionError::InvalidDiscriminatorLength { actual: data.len() });
    }

    let discriminator = data[0..8].try_into().map_err(|_| InstructionError::ParseDiscriminator)?;

    Ok(discriminator)
}

pub fn validate_discriminator(data: &[u8], discriminator: [u8; 8]) -> Result<()> {
    let ix_discriminator = get_discriminator(data)?;
    if ix_discriminator != discriminator {
        return Err(InstructionError::InvalidDiscriminator {
            expected: discriminator,
            actual: ix_discriminator,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_discriminator() {
        let data = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let discriminator = get_discriminator(&data).unwrap();
        assert_eq!(discriminator, [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    }
}
