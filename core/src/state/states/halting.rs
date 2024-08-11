/*
    Appellation: halting <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::{RawState, State};

#[doc(hidden)]
pub trait Haltable<Q = String> {
    const HALT: bool = true;

    type State: RawState<Ctx = Q>;

    private!();
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Halt<Q>(pub Q);

impl<Q> Halt<Q> {
    pub fn new(halt: Q) -> Self {
        Self(halt)
    }

    pub fn into_inner(self) -> Q {
        self.0
    }

    pub fn as_ref<'a>(&'a self) -> Halt<&'a Q> {
        Halt(&self.0)
    }

    pub fn as_mut<'a>(&'a mut self) -> Halt<&'a mut Q> {
        Halt(&mut self.0)
    }

    pub fn as_state<'a>(&'a self) -> State<Halt<&'a Q>> {
        State(Halt(&self.0))
    }

    pub fn cloned(&self) -> Self
    where
        Q: Clone,
    {
        Self(self.0.clone())
    }

    pub fn copied(&self) -> Self
    where
        Q: Copy,
    {
        Self(self.0)
    }
}

impl<Q> From<State<Q>> for Halt<Q> {
    fn from(State(state): State<Q>) -> Self {
        Self(state)
    }
}

impl<Q> Haltable<Q> for Halt<Q> {
    type State = State<Q>;
    seal!();
}

impl<Q> Haltable<Q> for State<Halt<Q>> {
    type State = State<Q>;
    seal!();
}
