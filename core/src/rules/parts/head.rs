/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::State;

/// The head of a turing machine generally speaks to the current state and symbol of the machine
/// w.r.t. the [tape](crate::types::Tape).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Head<Q = String, S = char> {
    pub(crate) state: State<Q>,
    pub(crate) symbol: S,
    // pub(crate) _ptr: ,
}

impl<Q, S> Head<Q, S> {
    pub fn new(State(state): State<Q>, symbol: S) -> Self {
        Self {
            state: State(state),
            symbol,
        }
    }
    /// Returns a reference to the current [state](State) and symbol returing a 2-tuple
    pub fn as_tuple(&self) -> (&State<Q>, &S) {
        (&self.state, &self.symbol)
    }
    /// Consumes the head and returns the current [state](State) and symbol as a 2-tuple
    pub fn into_tuple(self) -> (State<Q>, S) {
        (self.state, self.symbol)
    }
    /// Returns a mutable reference to the current [state](State) and symbol as a 2-tuple
    pub fn as_mut_tuple(&mut self) -> (&mut State<Q>, &mut S) {
        (&mut self.state, &mut self.symbol)
    }

    /// Updates the current [state](State)
    pub fn set_state(&mut self, state: State<Q>) {
        self.state = state;
    }
    /// Updates the current symbol
    pub fn set_symbol(&mut self, symbol: S) {
        self.symbol = symbol;
    }
    /// Returns a reference to the current [state](State)
    pub fn state(&self) -> State<&'_ Q> {
        self.state.to_view()
    }
    /// Returns a mutable reference to the current [state](State)
    pub fn state_mut(&mut self) -> &mut State<Q> {
        &mut self.state
    }
    /// Returns a reference to the current symbol
    pub const fn symbol(&self) -> &S {
        &self.symbol
    }
    /// Returns a mutable reference to the current symbol
    pub fn symbol_mut(&mut self) -> &mut S {
        &mut self.symbol
    }
    /// Updates the current [state](State) and symbol
    pub fn update(&mut self, state: Option<State<Q>>, symbol: Option<S>) {
        if let Some(state) = state {
            self.state = state;
        }
        if let Some(symbol) = symbol {
            self.symbol = symbol;
        }
    }
}

impl<'a, Q, S> Head<&'a Q, &'a S> {
    pub fn cloned(&self) -> Head<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Head {
            state: self.state.cloned(),
            symbol: self.symbol.clone(),
        }
    }
}

impl<Q, S> From<(State<Q>, S)> for Head<Q, S> {
    fn from((state, symbol): (State<Q>, S)) -> Self {
        Self::new(state, symbol)
    }
}

impl<Q, S> From<Head<Q, S>> for (State<Q>, S) {
    fn from(head: Head<Q, S>) -> Self {
        head.into_tuple()
    }
}
