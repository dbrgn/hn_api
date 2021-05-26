//! Errors, type aliases, and functions related to working with `Result`.

/// Result
pub type Result<T> = std::result::Result<T, Error>;

/// Represents all the ways that a request can fail.
#[derive(Debug)]
pub enum Error {
    /// ReqwestError
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(err)
    }
}
