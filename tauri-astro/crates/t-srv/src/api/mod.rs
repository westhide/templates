pub mod therscan;

use reqwest::Error as ReqwestError;
use url::ParseError as UrlParseError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    UrlParseError(#[from] UrlParseError),

    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),
}
