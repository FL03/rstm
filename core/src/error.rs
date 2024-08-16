/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, strum::VariantNames, thiserror::Error,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum StateError {
    #[error("Invalid State: {0}")]
    InvalidState(String),
    #[error("State Not Found")]
    StateNotFound,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, strum::VariantNames, thiserror::Error,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Error {
    #[error("[IndexError] Out of Bounds: {index} is out of bounds for a length of {len}")]
    IndexOutOfBounds { index: usize, len: usize },
    #[error("[Runtime] Runtime Error: {0}")]
    RuntimeError(String),
    #[error("State Error: {0}")]
    StateError(#[from] StateError),
    #[error("Transformation error: {0}")]
    TransformationError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl Error {
    pub fn index_out_of_bounds(index: usize, len: usize) -> Self {
        Error::IndexOutOfBounds { index, len }
    }

    pub fn runtime_error(message: impl ToString) -> Self {
        Error::RuntimeError(message.to_string())
    }

    pub fn state_error(err: StateError) -> Self {
        Error::StateError(err)
    }

    pub fn transformation_error(message: impl ToString) -> Self {
        Error::TransformationError(message.to_string())
    }

    pub fn unknown(message: impl ToString) -> Self {
        Error::Unknown(message.to_string())
    }

    pub fn invalid_state(err: impl ToString) -> Self {
        Error::StateError(StateError::InvalidState(err.to_string()))
    }

    pub fn state_not_found() -> Self {
        Error::StateError(StateError::StateNotFound)
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
