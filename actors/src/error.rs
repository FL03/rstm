/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This module defines the custom error type for handling various actor-related errors.
#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// A type alias for a [`Result`](core::result::Result) that uses the custom [`Error`] type
pub type Result<T> = core::result::Result<T, Error>;

/// the various errors that can occur in the state module
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The actor has halted.")]
    Halted,
    #[error(transparent)]
    CoreError(#[from] rstm_core::Error),
    #[error(transparent)]
    RulesError(#[from] rstm_rules::Error),
    #[error(transparent)]
    StateError(#[from] rstm_state::StateError),
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
