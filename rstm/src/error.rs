/*
    Appellation: error <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, strum::VariantNames, thiserror::Error,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum FsmError {
    #[error("[IndexError] Out of Bounds: {0}")]
    IndexOutOfBounds(String),
    #[error("[StateError] Invalid State: {0}")]
    InvalidState(String),
    #[error("[StateError] State Not Found: {0}")]
    StateNotFound(String),
    #[error("Transformation error: {0}")]
    TransformationError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl FsmError {
    pub fn index_out_of_bounds(err: impl ToString) -> Self {
        FsmError::IndexOutOfBounds(err.to_string())
    }

    pub fn invalid_state(err: impl ToString) -> Self {
        FsmError::InvalidState(err.to_string())
    }

    pub fn state_not_found(err: impl ToString) -> Self {
        FsmError::StateNotFound(err.to_string())
    }

    pub fn transformation_error(message: impl ToString) -> Self {
        FsmError::TransformationError(message.to_string())
    }

    pub fn unknown(message: impl ToString) -> Self {
        FsmError::Unknown(message.to_string())
    }
}
