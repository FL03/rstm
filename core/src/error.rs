/*
    Appellation: error <module>
    Contrib: @FL03
*/
//! this module defines the [`RuleError`] type alongisde other useful primitives that are used
//! throughout the rules crate.

/// A type alias for a [`Result`](core::result::Result) with a custom error type ([Error])
pub type RuleResult<T = ()> = core::result::Result<T, RuleError>;

/// [`RuleError`] enumerates the various errors encountered when dealing with rules and their
/// components.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RuleError {
    #[error("An invalid direction was specified.")]
    InvalidDirection,
    #[error("Unable to downcast into the specified type.")]
    DowncastFailure,
    #[error("No symbol found at position {0}")]
    NoSymbolFoundAt(usize),
    #[error(transparent)]
    AnyError(#[from] anyhow::Error),
    #[cfg(feature = "alloc")]
    #[error(transparent)]
    BoxError(#[from] alloc::boxed::Box<dyn core::error::Error + Send + Sync>),
    #[cfg(feature = "serde")]
    #[error(transparent)]
    DeserializeError(#[from] serde::de::value::Error),
    #[error(transparent)]
    FmtError(#[from] core::fmt::Error),
    #[cfg(feature = "std")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[cfg(feature = "json")]
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

impl RuleError {
    #[cfg(feature = "alloc")]
    /// a utility function facilitating the creation of the [`BoxError`](RuleError::BoxError)
    /// variant
    pub fn box_error<E>(e: E) -> Self
    where
        E: 'static + core::error::Error + Send + Sync,
    {
        Self::BoxError(alloc::boxed::Box::new(e))
    }
}
