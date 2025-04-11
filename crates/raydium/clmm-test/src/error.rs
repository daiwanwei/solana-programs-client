use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClmmTestError {
    #[error("Client error: {0}")]
    ClientError(#[from] raydium_clmm_client::error::ClmmClientError),

    #[error("Failed transaction metadata")]
    FailedTransactionMetadata,

    #[error("Token error: {0}")]
    TokenError(#[from] program_test_utils::token::TokenError),

    #[error("Pool state not found")]
    PoolStateNotFound,

    #[error("Tick array bitmap not found")]
    TickArrayBitmapNotFound,

    #[error("Personal position not found")]
    PersonalPositionNotFound,

    #[error("Mint not found")]
    MintNotFound,

    #[error("Invalid token pair")]
    InvalidTokenPair,

    #[error("Tick array not found")]
    TickArrayNotFound,

    #[error("Token account not found")]
    TokenAccountNotFound,
}

impl From<litesvm::types::FailedTransactionMetadata> for ClmmTestError {
    fn from(_: litesvm::types::FailedTransactionMetadata) -> Self {
        Self::FailedTransactionMetadata
    }
}

pub type Result<T> = std::result::Result<T, ClmmTestError>;
