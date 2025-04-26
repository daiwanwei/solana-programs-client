use solana_deserialize::account;
use solana_rpc_client_api::client_error::Error as RpcError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountError {
    #[error("Failed to deserialize account: {source}")]
    Deserialize { source: account::AccountError },

    #[error("Failed to fetch account: {source}")]
    FetchAccount { source: RpcError },

    #[error("Too many pubkeys: expected {max}, actual {actual}")]
    TooManyPubkeys { max: usize, actual: usize },
}

pub type Result<T> = std::result::Result<T, AccountError>;
