/*
    Appellation: wrap <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::{Halt, State};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Halting<Q> {
    Step(State<Q>),
    Halt(Halt<Q>),
}

impl<Q> Halting<Q> {
    pub fn is_halted(&self) -> bool {
        match self {
            Self::Halt(_) => true,
            _ => false,
        }
    }

    pub fn is_continuing(&self) -> bool {
        match self {
            Self::Step(_) => true,
            _ => false,
        }
    }

    pub fn into_state(self) -> State<Q> {
        match self {
            Self::Step(state) => state,
            Self::Halt(halt) => State(halt.0),
        }
    }

    pub fn as_state(&self) -> State<&Q> {
        match self {
            Self::Step(state) => state.to_ref(),
            Self::Halt(halt) => State(halt.view().0),
        }
    }

    pub fn get(&self) -> &Q {
        match self {
            Self::Step(inner) => inner.get(),
            Self::Halt(inner) => inner.get(),
        }
    }

    pub fn get_mut(&mut self) -> &mut Q {
        match self {
            Self::Step(inner) => inner.get_mut(),
            Self::Halt(inner) => inner.get_mut(),
        }
    }

    pub fn set(&mut self, state: Q) {
        match self {
            Self::Step(inner) => inner.set(state),
            Self::Halt(inner) => inner.set(state),
        }
    }
}

impl<Q: Default> Default for Halting<Q> {
    fn default() -> Self {
        Self::Step(State::default())
    }
}
