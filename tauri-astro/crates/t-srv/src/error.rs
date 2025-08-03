use std::{env::VarError as StdEnvVarError, io::Error as StdIoError, sync::PoisonError};

use alloy::{
    hex::FromHexError, primitives::utils::UnitsError, sol_types::Error as AlloySolTypesError,
};
use axum::response::{IntoResponse, Response};
use foundry_block_explorers::errors::EtherscanError;
use serde_json::Error as SerdeJsonError;
use t_lib::error::Error as TLibError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    StdIoError(#[from] StdIoError),

    #[error(transparent)]
    StdEnvVarError(#[from] StdEnvVarError),

    #[error(transparent)]
    EtherscanError(#[from] EtherscanError),

    #[error(transparent)]
    AlloySolTypesError(#[from] AlloySolTypesError),

    #[error(transparent)]
    FromHexError(#[from] FromHexError),

    #[error(transparent)]
    UnitsError(#[from] UnitsError),

    #[error(transparent)]
    SerdeJsonError(#[from] SerdeJsonError),

    #[error("{0}")]
    Generic(String),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(err: PoisonError<T>) -> Self {
        Self::Generic(err.to_string())
    }
}

impl From<Error> for TLibError {
    fn from(err: Error) -> Self {
        Self::Generic(err.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        todo!()
    }
}

macro_rules! err {
    ($($arg:tt)*) => {
        Err($crate::error::Error::Generic(format!($($arg)*)))
    }
}

#[allow(unused_imports)]
pub(crate) use err;

pub type Result<T, E = Error> = std::result::Result<T, E>;
