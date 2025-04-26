use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Invalid discriminator length: expected 8, actual {actual}")]
    InvalidDiscriminatorLength { actual: usize },

    #[error("Failed to parse discriminator")]
    ParseDiscriminator,

    #[error("Invalid discriminator: expected {expected:?}, actual {actual:?}")]
    InvalidDiscriminator { expected: [u8; 8], actual: [u8; 8] },

    #[error("Failed to deserialize account: {source}")]
    DeserializeAnchorAccount { source: io::Error },

    #[error("Failed to deserialize account")]
    DeserializeSolanaAccount,

    #[error("Failed to decode account")]
    DecodeSolanaAccount,
}

pub type Result<T> = std::result::Result<T, AccountError>;
