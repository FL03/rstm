/*
    Appellation: halting <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::{halt::Haltable, State};

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

    pub const fn get(&self) -> &Q {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Q {
        &mut self.0
    }

    pub fn set(&mut self, halt: Q) {
        self.0 = halt;
    }
    /// Converts the halted state into a new [State] with an immutable reference to the inner value.
    pub fn as_state(&self) -> State<Halt<&Q>> {
        State(Halt(&self.0))
    }
    /// Converts the halted state into a new [State] with a mutable reference to the inner value.
    pub fn as_state_mut(&mut self) -> State<Halt<&mut Q>> {
        State(Halt(&mut self.0))
    }
    /// Wraps the halted state and returns a new [State]
    pub fn into_state(self) -> State<Halt<Q>> {
        State(self)
    }
    /// Returns an instance of [`Halt`] with an immutable reference to the inner value.
    pub fn view<'a>(&'a self) -> Halt<&'a Q> {
        Halt(&self.0)
    }
    /// Returns an instance of [`Halt`] with a mutable reference to the inner value.
    pub fn view_mut<'a>(&'a mut self) -> Halt<&'a mut Q> {
        Halt(&mut self.0)
    }
}

impl<'a, Q> Halt<&'a Q> {
    pub fn cloned(&self) -> Halt<Q>
    where
        Q: Clone,
    {
        Halt(self.0.clone())
    }

    pub fn copied(&self) -> Halt<Q>
    where
        Q: Copy,
    {
        Halt(*self.0)
    }
}

impl<'a, Q> Halt<&'a mut Q> {
    pub fn cloned(&self) -> Halt<Q>
    where
        Q: Clone,
    {
        Halt(self.0.clone())
    }

    pub fn copied(&self) -> Halt<Q>
    where
        Q: Copy,
    {
        Halt(*self.0)
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
