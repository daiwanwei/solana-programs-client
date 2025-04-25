use thiserror::Error;

pub type Result<T> = std::result::Result<T, WhirlpoolsTestError>;

#[derive(Error, Debug)]
pub enum WhirlpoolsTestError {
    #[error("Invalid program ID")]
    InvalidProgramId,

    #[error("Mint not found")]
    MintNotFound,

    #[error("Position not found")]
    PositionNotFound,

    #[error("Whirlpool not found")]
    WhirlpoolNotFound,

    #[error("Invalid tick arrays")]
    InvalidTickArrays,

    #[error("Token account not found")]
    TokenAccountNotFound,

    #[error("Whirlpools client error: {0}")]
    WhirlpoolsClientError(#[from] orca_whirlpools_client::error::WhirlpoolsClientError),

    #[error("Failed transaction metadata")]
    FailedTransactionMetadata,

    #[error("Program account not found")]
    ProgramAccountNotFound,
}

impl From<litesvm::types::FailedTransactionMetadata> for WhirlpoolsTestError {
    fn from(_: litesvm::types::FailedTransactionMetadata) -> Self {
        Self::FailedTransactionMetadata
    }
}
