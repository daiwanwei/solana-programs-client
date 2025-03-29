use litesvm::types::FailedTransactionMetadata;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClmmTestError {
    #[error("Pool state not found")]
    PoolStateNotFound,

    #[error("Failed transaction")]
    FailedTransaction(FailedTransactionMetadata),

    #[error("Mint not found")]
    MintNotFound,

    #[error("Test utils error")]
    TokenUtil(#[from] program_test_utils::token::TokenError),

    #[error("Svm lock error")]
    SvmLock,

    #[error("Personal position not found")]
    PersonalPositionNotFound,
}

impl From<FailedTransactionMetadata> for ClmmTestError {
    fn from(value: FailedTransactionMetadata) -> Self { Self::FailedTransaction(value) }
}

pub type Result<T> = std::result::Result<T, ClmmTestError>;
