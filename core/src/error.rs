/*
    Appellation: error <module>
    Contrib: @FL03
*/
//! this module defines the [`Error`] type alongisde other useful primitives that are used
//! throughout the rules crate.
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String};

/// A type alias for a [`Result`](core::result::Result) with a custom error type ([Error])
pub type Result<T = ()> = core::result::Result<T, Error>;

/// [`Error`] enumerates the various errors encountered when dealing with rules and their
/// components.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    // custom errors
    #[error("No program or ruleset has been loaded into the actor.")]
    NoProgram,
    #[error("No rule found associated with the current state and symbol.")]
    NoRuleFound,
    #[error("Index {index} is out of bounds for length {len}.")]
    IndexOutOfBounds { index: usize, len: usize },
    #[error("A halted machine attempted to perform an operation.")]
    Halted,
    #[error("Unable to parse a rule from the given information.")]
    ParseRuleError,
    #[error("An invalid direction was specified.")]
    InvalidDirection,
    #[error("Unable to downcast into the specified type.")]
    DowncastFailure,
    #[error("No symbol found at position {0}")]
    NoSymbolFoundAt(usize),
    // internal errors
    #[error(transparent)]
    StateError(#[from] rstm_state::StateError),
    // external errors
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[cfg(feature = "serde")]
    #[error(transparent)]
    DeserializeError(#[from] serde::de::value::Error),
    #[cfg(feature = "json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    // Core Errors
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[error(transparent)]
    NetworkAddressParseError(#[from] core::net::AddrParseError),
    #[error(transparent)]
    Utf8Error(#[from] core::str::Utf8Error),
    // alloc-dependendent errors
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync>),
    #[cfg(feature = "alloc")]
    #[error("An unknown error occurred: {0}")]
    Unknown(String),
    // std-dependend errors
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

#[cfg(feature = "alloc")]
impl Error {
    /// a utility function facilitating the creation of the [`BoxError`](Error::BoxError)
    /// variant
    pub fn box_error<E>(e: E) -> Self
    where
        E: 'static + core::error::Error + Send + Sync,
    {
        Self::BoxError(Box::new(e))
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Unknown(String::from(s))
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Unknown(s)
    }
}
