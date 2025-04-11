use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClmmClientError {
    #[error("Failed to prepare instruction")]
    InstructionPreparationFailed,
    #[error("Invalid parameter")]
    InvalidParameter,
    #[error("Account not found")]
    AccountNotFound,
    #[error("State not found")]
    StateNotFound,
}

pub type Result<T> = std::result::Result<T, ClmmClientError>;
