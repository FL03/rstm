/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// A type alias for a [Result] with our custom error type: [`Error`](crate::Error)
pub type Result<T = ()> = core::result::Result<T, crate::Error>;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, strum::EnumIs, thiserror::Error)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum StateError {
    #[error("Invalid State: {0}")]
    InvalidState(String),
    #[error("State Not Found")]
    StateNotFound,
}

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    scsys_derive::VariantConstructors,
    strum::EnumDiscriminants,
    strum::EnumIs,
    thiserror::Error,
)]
#[strum_discriminants(derive(Hash, Ord, PartialOrd))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Error {
    #[error("[Execution Error] {0}")]
    ExecutionError(String),
    #[error("[Index Error] Out of Bounds: {index} is out of bounds for a length of {len}")]
    IndexOutOfBounds { index: usize, len: usize },
    #[error("[Runtime Error] {0}")]
    RuntimeError(String),
    #[error("[State Error] {0}")]
    StateError(#[from] StateError),
    #[error("[Transformation Error]: {0}")]
    TransformationError(String),
    #[error("[Type Error] {0}")]
    TypeError(String),
    #[error("[Unknown Error] {0}")]
    Unknown(String),
}

impl From<Box<dyn std::error::Error>> for Error {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Error::Unknown(err.to_string())
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Unknown(err.to_string())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Unknown(err)
    }
}
