use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstructionError {
    #[error("Invalid discriminator length: expected 8, actual {actual}")]
    InvalidDiscriminatorLength { actual: usize },

    #[error("Failed to parse discriminator")]
    ParseDiscriminator,

    #[error("Invalid discriminator: expected {expected:?}, actual {actual:?}")]
    InvalidDiscriminator { expected: [u8; 8], actual: [u8; 8] },

    #[error("Failed to deserialize instruction: {source}")]
    DeserializeInstruction { source: io::Error },
}

pub type Result<T> = std::result::Result<T, InstructionError>;
