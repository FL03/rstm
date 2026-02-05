/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This module defines the custom error type for handling various state-related errors.
#[cfg(feature = "alloc")]
use alloc::string::String;

/// A type alias for a [`Result`](core::result::Result) that uses the custom [`StateError`] type
pub type Result<T> = core::result::Result<T, StateError>;

/// the various errors that can occur in the state module
#[derive(Debug, thiserror::Error)]
pub enum StateError {
    #[error("Failed to downcast state")]
    DowncastFailure,
    #[cfg(feature = "alloc")]
    #[error("Invalid State: {0}")]
    InvalidState(String),
    #[error("State Not Found")]
    StateNotFound,
}
