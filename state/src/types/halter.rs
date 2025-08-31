/*
    Appellation: halting <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::State;
use crate::traits::{Halt, RawState};

/// [`Halter`] extends the [State] by allowing for an 'imaginary' state that is not actually
/// part of the machine's state space.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::EnumDiscriminants,
    strum::EnumIs,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase"),
    strum_discriminants(derive(serde::Deserialize, serde::Serialize))
)]
#[strum_discriminants(name(HaltTag), derive(Hash, Ord, PartialOrd))]
pub enum Halter<Q = usize> {
    Halt(Q),
    State(Q),
}

impl<Q> Halter<Q>
where
    Q: RawState,
{
    /// Creates a new instance of a [HaltState] with a halted state.
    pub fn halt(State(state): State<Q>) -> Self {
        Self::Halt(state)
    }
    /// Creates a new instance of a [HaltState] with a continuing state.
    pub fn state(State(state): State<Q>) -> Self {
        Self::State(state)
    }

    pub fn into_state(self) -> State<Q> {
        match self {
            Self::State(state) => State(state),
            Self::Halt(halt) => State(halt),
        }
    }

    pub fn as_state(&self) -> State<&Q> {
        State(self.get())
    }

    pub fn as_mut_state(&mut self) -> State<&mut Q> {
        State(self.get_mut())
    }

    pub fn get(&self) -> &Q {
        match self {
            Self::State(inner) => inner,
            Self::Halt(inner) => inner,
        }
    }

    pub fn get_mut(&mut self) -> &mut Q {
        match self {
            Self::State(inner) => inner,
            Self::Halt(inner) => inner,
        }
    }

    pub fn set(&mut self, state: Q) -> &mut Self {
        match self {
            Self::State(inner) => {
                *inner = state;
            }
            Self::Halt(inner) => {
                *inner = state;
            }
        }
        self
    }
}

impl<Q> Default for Halter<Q>
where
    Q: Default,
{
    fn default() -> Self {
        Self::State(Default::default())
    }
}

impl<Q> From<State<Q>> for Halter<Q> {
    fn from(State(state): State<Q>) -> Self {
        Self::State(state)
    }
}

impl<Q> RawState for Halter<Q>
where
    Q: RawState,
{
    seal!();
}

impl<Q> Halt for Halter<Q>
where
    Q: RawState,
{
    seal!();

    fn is_halted(&self) -> bool {
        matches!(self, Self::Halt(_))
    }
}
