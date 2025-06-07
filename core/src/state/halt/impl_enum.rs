/*
    Appellation: wrap <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Halt, HaltState};
use crate::state::State;

impl<Q> HaltState<Q> {
    /// Creates a new instance of a [HaltState] with a halted state.
    pub fn halt(Halt(state): Halt<Q>) -> Self {
        Self::Halt(Halt(state))
    }
    /// Creates a new instance of a [HaltState] with a continuing state.
    pub fn state(state: State<Q>) -> Self {
        Self::State(state)
    }

    pub fn into_state(self) -> State<Q> {
        match self {
            Self::State(state) => state,
            Self::Halt(halt) => State(halt.0),
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
            Self::State(inner) => inner.get(),
            Self::Halt(inner) => inner.get_ref(),
        }
    }

    pub fn get_mut(&mut self) -> &mut Q {
        match self {
            Self::State(inner) => inner.get_mut(),
            Self::Halt(inner) => inner.get_mut(),
        }
    }

    pub fn set(&mut self, state: Q) {
        match self {
            Self::State(inner) => {
                let _ = inner.set(state);
            }
            Self::Halt(inner) => {
                inner.set(state);
            }
        }
    }
}

impl<Q> Default for HaltState<Q>
where
    Q: Default,
{
    fn default() -> Self {
        Self::State(State::default())
    }
}

impl<Q> From<State<Q>> for HaltState<Q> {
    fn from(state: State<Q>) -> Self {
        Self::State(state)
    }
}

impl<Q> From<Halt<Q>> for HaltState<Q> {
    fn from(halt: Halt<Q>) -> Self {
        Self::Halt(halt)
    }
}
