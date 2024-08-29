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
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::EnumDiscriminants,
    strum::VariantNames,
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
    #[error("[Unknown Error] {0}")]
    Unknown(String),
}

impl Error {
    pub fn execution_error(message: impl ToString) -> Self {
        Error::ExecutionError(message.to_string())
    }

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

    pub fn message(&self) -> String {
        match self {
            Error::ExecutionError(message) => message.clone(),
            Error::IndexOutOfBounds { index, len } => {
                format!(
                    "Out of Bounds: {} is out of bounds for a length of {}",
                    index, len
                )
            }
            Error::RuntimeError(message) => message.clone(),
            Error::StateError(err) => err.to_string(),
            Error::TransformationError(message) => message.clone(),
            Error::Unknown(message) => message.clone(),
        }
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
