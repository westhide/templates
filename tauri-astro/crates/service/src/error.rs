use std::{
    env::VarError as StdEnvVarError, io::Error as StdIoError,
    net::AddrParseError as StdNetAddrParseError,
};

use tauri::Error as TauriError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    StdEnvVarError(#[from] StdEnvVarError),

    #[error(transparent)]
    StdIoError(#[from] StdIoError),

    #[error(transparent)]
    StdNetAddrParseError(#[from] StdNetAddrParseError),

    #[error(transparent)]
    TauriError(#[from] TauriError),

    #[error("{0}")]
    Generic(String),
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::Generic(err)
    }
}

macro_rules! err {
    ($($arg:tt)*) => {
        Err($crate::error::Error::Generic(format!($($arg)*)))
    }
}

pub(crate) use err;

pub type Result<T, E = Error> = std::result::Result<T, E>;
