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
    #[error("No tick array available")]
    NoTickArrayAvailable,
    #[error("Swap error: {0}")]
    SwapError(#[from] raydium_clmm::math::swap_v2::SwapError),
}

pub type Result<T> = std::result::Result<T, ClmmClientError>;
