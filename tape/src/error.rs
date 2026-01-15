/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This module defines the custom error type for handling various state-related errors.

/// A type alias for a [`Result`](core::result::Result) that uses the custom [`Error`] type
pub type Result<T> = core::result::Result<T, TapeError>;

/// the various errors that can occur in the state module
#[derive(Debug, thiserror::Error)]
pub enum TapeError {
    #[error("Unable to allocate memory")]
    AllocationFailure,
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),
}
