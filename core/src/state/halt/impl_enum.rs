/*
    Appellation: wrap <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Halt;
use crate::state::{RawState, State};

impl<Q> Halt<Q>
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

impl<Q> Default for Halt<Q>
where
    Q: Default,
{
    fn default() -> Self {
        Self::State(Default::default())
    }
}

impl<Q> From<State<Q>> for Halt<Q> {
    fn from(State(state): State<Q>) -> Self {
        Self::State(state)
    }
}
