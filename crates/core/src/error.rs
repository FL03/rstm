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
    #[error("Unable to index into the given structure.")]
    IndexError,
    #[error("No inputs were provided for the program.")]
    TapeIsEmpty,
    #[error("The runtime exited without halting")]
    ExitWithoutHalting,
    #[error("Unable to find anything to read from the current position of the head.")]
    NothingToRead,
    #[error("No program or ruleset has been loaded into the actor.")]
    NoProgram,
    #[error("No rule found associated with the current state and symbol.")]
    NoRuleFound,
    #[error("The index ({idx}) is out of bounds for a tape of length {len}")]
    IndexOutOfBounds { idx: usize, len: usize },
    #[error("Attempted to perform an operation in a halted state.")]
    Halted,
    #[error("Unable to parse a rule from the given information.")]
    ParseRuleError,
    #[error("An invalid direction was specified.")]
    InvalidDirection,
    #[error("Unable to downcast {0:?} into type {1:?}.")]
    DowncastFailure(core::any::TypeId, core::any::TypeId),
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

impl Error {
    /// a functional constructor for the [`IndexOutOfBounds`](Error::IndexOutOfBounds) variant
    pub const fn index_out_of_bounds(idx: usize, len: usize) -> Self {
        Self::IndexOutOfBounds { idx, len }
    }
    #[cfg(feature = "alloc")]
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

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumString,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[strum(serialize_all = "snake_case")]
pub enum ErrorKind {
    Empty,
    IOError,
    IndexError,
    ExitWithoutHalting,
    NothingToRead,
    NoProgram,
    NoRuleFound,
    IndexOutOfBounds,
    Halted,
    ParseRuleError,
    InvalidDirection,
    DowncastFailure,
    NoSymbolFoundAt,
    StateError,
    AnyError,
    DeserializeError,
    JsonError,
    FmtError,
    NetworkAddressParseError,
    Utf8Error,
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct ErrorBase<E> {
    pub kind: ErrorKind,
    pub error: E,
}
