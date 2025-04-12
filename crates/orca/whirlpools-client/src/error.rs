use thiserror::Error;

pub type Result<T> = std::result::Result<T, WhirlpoolsClientError>;

#[derive(Error, Debug)]
pub enum WhirlpoolsClientError {
    #[error("Invalid program ID")]
    InvalidProgramId,
}
