/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
mod impl_head;
mod impl_head_repr;

#[allow(deprecated)]
mod impl_deprecated;

use rstm_state::{RawState, State};

pub type HeadRef<'a, Q, S> = Head<&'a Q, &'a S>;
/// a head with mutable references to its state and symbol
pub type HeadMut<'a, Q, S> = Head<&'a mut Q, &'a mut S>;

/// The [Head] is formally defined to be a 2-tuple consisting of a state / symbol pair.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct Head<Q, S> {
    #[cfg_attr(feature = "serde", serde(alias = "current_state"))]
    pub state: State<Q>,
    #[cfg_attr(feature = "serde", serde(alias = "current_symbol"))]
    pub symbol: S,
}

impl<Q, S> Head<Q, S>
where
    Q: RawState,
{
    /// a functional constructor for the [`Head`]
    pub const fn new(state: Q, symbol: S) -> Self {
        Self {
            state: State(state),
            symbol,
        }
    }
    /// returns a new instance of the head using the given state and a default symbol
    pub fn from_state(state: Q) -> Self
    where
        S: Default,
    {
        Self::new(state, <S>::default())
    }
    /// returns a new instance of the head using the given symbol and a default state
    pub fn from_symbol(symbol: S) -> Self
    where
        Q: Default,
    {
        Self::new(<Q>::default(), symbol)
    }
    /// Create a new instance from a 2-tuple: `(state, symbol)`
    pub fn from_tuple((state, symbol): (State<Q>, S)) -> Self {
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
    pub fn with_symbol(self, symbol: S) -> Self {
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
    pub const fn symbol(&self) -> &S {
        &self.symbol
    }
    /// returns a mutable reference to the current symbol
    pub const fn symbol_mut(&mut self) -> &mut S {
        &mut self.symbol
    }
    /// returns a reference to the current state and symbol returing a 2-tuple
    pub const fn as_tuple(&self) -> (&State<Q>, &S) {
        (&self.state, &self.symbol)
    }
    /// Consumes the head and returns the current state and symbol as a 2-tuple
    pub fn into_tuple(self) -> (State<Q>, S) {
        (self.state, self.symbol)
    }
    /// returns a mutable reference to the current state and symbol as a 2-tuple
    pub const fn as_mut_tuple(&mut self) -> (&mut State<Q>, &mut S) {
        (&mut self.state, &mut self.symbol)
    }
    /// updates the current state
    pub fn set_state(&mut self, state: Q) {
        self.state_mut().set(state)
    }
    /// updates the current symbol
    pub fn set_symbol(&mut self, symbol: S) {
        self.symbol = symbol;
    }
    /// Replaces the current state and symbol with the given state and symbol; returns the
    /// previous instance of the head.
    pub const fn replace(&mut self, state: State<Q>, symbol: S) -> Self {
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
    pub const fn replace_symbol(&mut self, symbol: S) -> S {
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
    pub fn update(&mut self, state: Option<State<Q>>, symbol: Option<S>) {
        if let Some(state) = state {
            self.state = state;
        }
        if let Some(symbol) = symbol {
            self.symbol = symbol;
        }
    }

    /// returns a new head with immutable references to the current state and symbol
    pub const fn view(&self) -> Head<&Q, &S> {
        Head {
            state: self.state().view(),
            symbol: self.symbol(),
        }
    }
    /// returns a new head with mutable references to the current state and symbol
    pub const fn view_mut(&mut self) -> Head<&mut Q, &mut S> {
        Head {
            state: self.state.view_mut(),
            symbol: &mut self.symbol,
        }
    }
    /// tries reading the given tape using the head as its coordinates.
    pub fn read<T>(self, tape: &'_ [T]) -> Option<&<S>::Output>
    where
        S: core::slice::SliceIndex<[T]>,
    {
        tape.get(self.symbol)
    }
}
