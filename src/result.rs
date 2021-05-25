//! Errors, type aliases, and functions related to working with `Result`.

use thiserror::Error;

/// Result
pub type Result<T> = std::result::Result<T, Error>;

/// Represents all the ways that a request can fail.
#[derive(Error, Debug)]
pub enum Error {
    /// ReqwestError
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}
