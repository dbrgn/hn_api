pub mod blocking;
pub mod nonblocking;
pub mod types;

pub use blocking::HnClient;
pub use types::*;

#[derive(Debug)]
pub enum HnClientError {
    ItemNotFoundError(u32),
    UserNotFoundError(String),
    BackendError(String),
}

pub type Result<T> = std::result::Result<T, HnClientError>;

impl From<reqwest::Error> for HnClientError {
    fn from(err: reqwest::Error) -> Self {
        HnClientError::BackendError(err.to_string())
    }
}
