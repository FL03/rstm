/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "alloc")]
use alloc::string::String;

/// the various errors that can occur in the state module
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to downcast state")]
    DowncastError,
    #[cfg(feature = "alloc")]
    #[error("Invalid State: {0}")]
    InvalidState(String),
    #[error("State Not Found")]
    StateNotFound,
}
