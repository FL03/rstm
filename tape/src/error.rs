/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This module defines the custom error type for handling various tape-related errors.
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String};

/// A type alias for a [`Result`](core::result::Result) that uses the custom [`Error`] type
pub type Result<T> = core::result::Result<T, Error>;

/// The [`Error`] type enumerates various errors that can occur in tape operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(
        "The actor attempted to access an index ({index}) outside the bounds of the tape (size: {len})."
    )]
    IndexOutOfBounds { index: usize, len: usize },
    #[error(transparent)]
    CoreError(#[from] rstm_core::Error),
    #[error(transparent)]
    StateError(#[from] rstm_state::StateError),
    #[error("An unknown error was thrown by an actor: {0}")]
    UnknwonError(String),
}

#[cfg(feature = "alloc")]
impl From<Error> for rstm_core::Error {
    fn from(err: Error) -> Self {
        match err {
            Error::CoreError(e) => e,
            e => rstm_core::Error::BoxError(Box::new(e)),
        }
    }
}
