use std::sync::PoisonError;

use alloy::hex::FromHexError;
use foundry_block_explorers::errors::EtherscanError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    EtherscanError(#[from] EtherscanError),

    #[error(transparent)]
    FromHexError(#[from] FromHexError),

    #[error("{0}")]
    Generic(String),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<Error> for t_lib::error::Error {
    fn from(err: Error) -> Self {
        Self::Generic(err.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
