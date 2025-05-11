use thiserror::Error;

#[derive(Error, Debug)]
pub enum LighthouseClientError {
    #[error("Invalid memory ID")]
    InvalidMemoryId(u8),
}

pub type Result<T> = std::result::Result<T, LighthouseClientError>;
