/*
    Appellation: impl_halting <module>
    Contrib: @FL03
*/
use super::Halt;
use crate::state::{RawState, State};

impl<Q> Halt<Q> {
    pub fn new(halt: Q) -> Self {
        Self(halt)
    }
    #[inline]
    /// Consumes the halted state and returns the inner value.
    pub fn get(self) -> Q {
        self.0
    }
    /// Returns an immutable reference to the inner value of the halted state.
    pub const fn get_ref(&self) -> &Q {
        &self.0
    }
    /// Returns a mutable reference to the inner value of the halted state.
    pub fn get_mut(&mut self) -> &mut Q {
        &mut self.0
    }
    /// Replaces the inner value of the halted state with the given value, returning the
    /// previous value.
    pub fn replace(&mut self, halt: Q) -> Q {
        core::mem::replace(&mut self.0, halt)
    }
    /// Resets the inner value of the halted state to the default value of the type.
    pub fn reset(&mut self)
    where
        Q: Default,
    {
        self.set(Default::default());
    }
    /// Sets the inner value of the halted state to that of the given value.
    pub fn set(&mut self, halt: Q) {
        self.0 = halt;
    }
    /// Swaps the inner value of the halted state with that of the given state.
    pub fn swap<S>(&mut self, other: &mut S)
    where
        S: RawState<Q = Q>,
    {
        core::mem::swap(&mut self.0, other.get_mut());
    }
    /// Takes the inner value of the halted state and replaces it with the default value of
    /// the type.
    pub fn take(&mut self) -> Q
    where
        Q: Default,
    {
        core::mem::take(&mut self.0)
    }
    /// Converts the halted state into a new [State] with an immutable reference to the inner
    /// value.
    pub fn as_state(&self) -> State<Halt<&Q>> {
        State(Halt(&self.0))
    }
    /// Converts the halted state into a new [State] with a mutable reference to the inner
    /// value.
    pub fn as_state_mut(&mut self) -> State<Halt<&mut Q>> {
        State(Halt(&mut self.0))
    }
    /// Wraps the halted state and returns a new [State]
    pub fn into_state(self) -> State<Halt<Q>> {
        State(self)
    }
    /// Returns an instance of [`Halt`] with an immutable reference to the inner value.
    pub fn view(&self) -> Halt<&Q> {
        Halt(&self.0)
    }
    /// Returns an instance of [`Halt`] with a mutable reference to the inner value.
    pub fn view_mut(&mut self) -> Halt<&mut Q> {
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

impl<Q> From<Halt<Q>> for State<Q> {
    fn from(Halt(state): Halt<Q>) -> Self {
        Self(state)
    }
}
