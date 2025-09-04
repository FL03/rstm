/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This module defines the custom error type for handling various actor-related errors.
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String};

/// A type alias for a [`Result`](core::result::Result) that uses the custom [`Error`] type
pub type Result<T> = core::result::Result<T, Error>;

/// the various errors that can occur in the state module
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "alloc")]
    #[error("[Execution Error] {0}")]
    ExecutionError(String),
    #[error("The actor is halted and should be terminated.")]
    Halted,
    #[error("The actor is not ready to process inputs.")]
    NotReady,
    #[error("The actor has no program loaded.")]
    NoProgram,
    #[error("The actor has no inputs to process.")]
    NoInputs,
    #[error("An infinite loop was detected during execution.")]
    InfiniteLoop,
    #[error("No rule found")]
    NoRuleFound,
    #[error("No symbol found at position {0}.")]
    NoSymbolFound(usize),
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
