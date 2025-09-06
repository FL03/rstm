/*
    Appellation: impl_head <module>
    Created At: 2025.08.31:00:00:55
    Contrib: @FL03
*/
use super::{Head, HeadMut, HeadRef};
use crate::rule::Rule;
use crate::tail::Tail;
use rstm_state::{RawState, State};

/// The base implementation of the [`Head`] type focusing on providing constructors and basic
/// methods for manipulating its components.
impl<Q, A> Head<Q, A>
where
    Q: RawState,
{
    /// initialize a new instance of the [`Head`] given some state and symbol
    pub const fn new(state: Q, symbol: A) -> Self {
        Self {
            state: State(state),
            symbol,
        }
    }
    /// returns a new instance of the head using the given state and a default symbol
    pub fn from_state(state: Q) -> Self
    where
        A: Default,
    {
        Self::new(state, <A>::default())
    }
    /// returns a new instance of the head using the given symbol and a default state
    pub fn from_symbol(symbol: A) -> Self
    where
        Q: Default,
    {
        Self::new(<Q>::default(), symbol)
    }
    /// Create a new instance from a 2-tuple: `(state, symbol)`
    pub fn from_tuple((state, symbol): (State<Q>, A)) -> Self {
        Self { state, symbol }
    }
    /// consumes the current instance to create another instance with the given state
    pub fn with_state(self, state: Q) -> Self {
        Self {
            state: State(state),
            ..self
        }
    }
    /// consumes the current instance to create another instance with the given symbol
    pub fn with_symbol(self, symbol: A) -> Self {
        Self { symbol, ..self }
    }
    /// returns a reference to the current state
    pub const fn state(&self) -> &State<Q> {
        &self.state
    }
    /// returns a mutable reference to the current [State]
    pub const fn state_mut(&mut self) -> &mut State<Q> {
        &mut self.state
    }
    /// returns a reference to the current symbol
    pub const fn symbol(&self) -> &A {
        &self.symbol
    }
    /// returns a mutable reference to the current symbol
    pub const fn symbol_mut(&mut self) -> &mut A {
        &mut self.symbol
    }
    /// updates the current state
    pub fn set_state(&mut self, state: Q) {
        self.state_mut().set(state)
    }
    /// updates the current symbol
    pub fn set_symbol(&mut self, symbol: A) {
        self.symbol = symbol;
    }
    /// replaces the current values with the given state and symbol, returning the previous
    /// instance of the [`Head`]
    pub const fn replace(&mut self, state: State<Q>, symbol: A) -> Self {
        Head {
            state: self.replace_state(state),
            symbol: self.replace_symbol(symbol),
        }
    }
    /// [`replace`](core::mem::replace) the current state with the given state, returning the
    /// previous state
    pub const fn replace_state(&mut self, state: State<Q>) -> State<Q> {
        core::mem::replace(self.state_mut(), state)
    }
    /// [`replace`](core::mem::replace) the current symbol with the given symbol, returning the
    /// previous symbol
    pub const fn replace_symbol(&mut self, symbol: A) -> A {
        core::mem::replace(self.symbol_mut(), symbol)
    }
    /// [`swap`](core::mem::swap) the current state and symbol with those of the given head
    pub const fn swap(&mut self, other: &mut Self) {
        // swap the states
        core::mem::swap(self.state_mut(), other.state_mut());
        // swap the symbols
        core::mem::swap(self.symbol_mut(), other.symbol_mut());
    }
    /// updates the current [State] and symbol
    pub fn update(&mut self, state: Option<State<Q>>, symbol: Option<A>) {
        if let Some(s) = state {
            self.set_state(s.value())
        }
        if let Some(s) = symbol {
            self.set_symbol(s)
        }
    }
    /// returns a reference to the current state and symbol returing a 2-tuple
    pub const fn as_tuple(&self) -> (&State<Q>, &A) {
        (self.state(), self.symbol())
    }
    /// returns a mutable reference to the current state and symbol as a 2-tuple
    pub const fn as_mut_tuple(&mut self) -> (&mut State<Q>, &mut A) {
        (&mut self.state, &mut self.symbol)
    }
    /// Consumes the head and returns the current state and symbol as a 2-tuple
    pub fn into_tuple(self) -> (State<Q>, A) {
        (self.state, self.symbol)
    }
}
impl<Q, A> Head<Q, A>
where
    Q: RawState,
{
    /// associates the given tail with the current head, returning a new [`Rule`]
    pub fn append(self, tail: Tail<Q, A>) -> Rule<Q, A> {
        Rule { head: self, tail }
    }
    /// tries reading the given tape using the head as its coordinates.
    pub fn read<T>(self, tape: &'_ [T]) -> Option<&<A>::Output>
    where
        A: core::slice::SliceIndex<[T]>,
    {
        tape.get(self.symbol)
    }
    /// returns a new head with immutable references to the current state and symbol
    pub const fn view(&self) -> HeadRef<'_, Q, A> {
        Head {
            state: self.state().view(),
            symbol: self.symbol(),
        }
    }
    /// returns a new head with mutable references to the current state and symbol
    pub const fn view_mut(&mut self) -> HeadMut<'_, Q, A> {
        Head {
            state: self.state.view_mut(),
            symbol: &mut self.symbol,
        }
    }
}
