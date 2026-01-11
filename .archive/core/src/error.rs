/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! The [`error`](self) module defines the core [`Error`] type used throughout the library and
//! provides a convenient alias for [`Result`](core::result::Result) types.
#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String};

/// A type alias for a [`Result`](core::result::Result) with an error type of [`Error`]
pub type Result<T = ()> = core::result::Result<T, self::Error>;

/// The [`Error`] implementation describes the various errors that can occur within the library
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("The specified index ({index}) is out of bounds for a collection of {len} elements.")]
    IndexOutOfBounds { index: usize, len: usize },
    #[cfg(feature = "alloc")]
    #[error("An unknown error occurred: {0}")]
    Unknown(String),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] Box<dyn core::error::Error + Send + Sync + 'static>),
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    #[cfg(feature = "serde_json")]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    StateError(#[from] rstm_state::StateError),
}

impl Error {
    /// returns an [`IndedOutOfBounds`](Error::IndexOutOfBounds) variant
    pub const fn index_out_of_bounds(index: usize, len: usize) -> Self {
        Self::IndexOutOfBounds { index, len }
    }
}

#[cfg(feature = "alloc")]
impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Unknown(String::from(err))
    }
}

#[cfg(feature = "alloc")]
impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Unknown(err)
    }
}
