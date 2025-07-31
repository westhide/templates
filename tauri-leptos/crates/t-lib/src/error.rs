use std::{io::Error as StdIoError, net::AddrParseError as StdNetAddrParseError};

use http::uri::InvalidUri as HttpUriInvalidUri;
use tonic::Status as TonicStatus;
#[cfg(feature = "tonic-transport")]
use tonic::transport::Error as TonicTransportError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    StdIoError(#[from] StdIoError),

    #[error(transparent)]
    StdNetAddrParseError(#[from] StdNetAddrParseError),

    #[error(transparent)]
    HttpUriInvalidUri(#[from] HttpUriInvalidUri),

    #[error(transparent)]
    TonicStatus(#[from] TonicStatus),

    #[cfg(feature = "tonic-transport")]
    #[error(transparent)]
    TonicTransportError(#[from] TonicTransportError),

    #[error("{0}")]
    Generic(String),
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        Err($crate::error::Error::Generic(format!($($arg)*)))
    }
}

#[allow(unused_imports)]
pub(crate) use err;

pub type Result<T, E = Error> = std::result::Result<T, E>;
