//! Results

use thiserror::Error;

/// Result
pub type Result<T> = std::result::Result<T, Error>;

/// Error
#[derive(Error, Debug)]
pub enum Error {
    /// ReqwestError
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}
