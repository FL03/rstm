/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::State;

/// The [Head] struct represent the state and symbol of an actor at a given moment in time.
/// With respect to a Turing machine, the head defines the current state and symbol of the
/// machine. When associated with a direction the head becomes a tail, instructing the machine
/// to move, write, and transition to a new state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
pub struct Head<Q = String, S = char> {
    #[cfg_attr(feature = "serde", serde(alias = "current_state"))]
    pub state: State<Q>,
    #[cfg_attr(feature = "serde", serde(flatten, alias = "current_symbol"))]
    pub symbol: S,
}

impl<Q, S> Head<Q, S> {
    pub fn new(State(state): State<Q>, symbol: S) -> Self {
        Self {
            state: State(state),
            symbol,
        }
    }
    /// Create a new instance of the [Head] using the given state and default symbol.
    pub fn from_state(State(state): State<Q>) -> Self
    where
        S: Default,
    {
        Self {
            state: State(state),
            symbol: Default::default(),
        }
    }
    /// Create a new instance of the [Head] using the given symbol and default state.
    pub fn from_symbol(symbol: S) -> Self
    where
        Q: Default,
    {
        Self {
            state: Default::default(),
            symbol,
        }
    }
    /// Create a new instance from a 2-tuple: ([state](State), symbol)
    pub fn from_tuple((state, symbol): (State<Q>, S)) -> Self {
        Self { state, symbol }
    }
    /// Updates the current [state](State) and returns a new head
    pub fn with_state(self, State(state): State<Q>) -> Self {
        Self {
            state: State(state),
            ..self
        }
    }
    /// Updates the current symbol and returns a new head
    pub fn with_symbol(self, symbol: S) -> Self {
        Self { symbol, ..self }
    }
    /// Returns a reference to the current state and symbol returing a 2-tuple
    pub fn as_tuple(&self) -> (&State<Q>, &S) {
        (&self.state, &self.symbol)
    }
    /// Consumes the head and returns the current state and symbol as a 2-tuple
    pub fn into_tuple(self) -> (State<Q>, S) {
        (self.state, self.symbol)
    }
    /// Returns a mutable reference to the current state and symbol as a 2-tuple
    pub fn as_mut_tuple(&mut self) -> (&mut State<Q>, &mut S) {
        (&mut self.state, &mut self.symbol)
    }
    /// Updates the current state
    pub fn set_state(&mut self, state: State<Q>) {
        self.state = state;
    }
    /// Updates the current symbol
    pub fn set_symbol(&mut self, symbol: S) {
        self.symbol = symbol;
    }
    /// Returns a reference to the current state
    pub fn state(&self) -> State<&Q> {
        self.state.to_ref()
    }
    /// Returns a mutable reference to the current [State]
    pub fn state_mut(&mut self) -> State<&mut Q> {
        self.state.to_mut()
    }
    /// Returns a reference to the current symbol
    pub const fn symbol(&self) -> &S {
        &self.symbol
    }
    /// Returns a mutable reference to the current symbol
    pub fn symbol_mut(&mut self) -> &mut S {
        &mut self.symbol
    }
    /// Updates the current [State] and symbol
    pub fn update(&mut self, state: Option<State<Q>>, symbol: Option<S>) {
        if let Some(state) = state {
            self.state = state;
        }
        if let Some(symbol) = symbol {
            self.symbol = symbol;
        }
    }
    /// Converts the current head into a new head with immutable references to the current state and symbol
    pub fn to_ref(&self) -> Head<&Q, &S> {
        Head {
            state: self.state.to_ref(),
            symbol: &self.symbol,
        }
    }
    /// Converts the current head into a new head with mutable references to the current state and symbol
    pub fn to_mut(&mut self) -> Head<&mut Q, &mut S> {
        Head {
            state: self.state.to_mut(),
            symbol: &mut self.symbol,
        }
    }

    pub fn read<T>(self, tape: &'_ [T]) -> Option<&<S>::Output>
    where
        S: core::slice::SliceIndex<[T]>,
    {
        tape.get(self.symbol)
    }
}

impl<Q> Head<Q, usize> {
    pub fn shift(self, direction: crate::Direction) -> Self {
        Self {
            symbol: direction.apply(self.symbol),
            ..self
        }
    }

    pub fn shift_inplace(&mut self, direction: crate::Direction) {
        self.symbol = direction.apply(self.symbol);
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

    pub fn copied(&self) -> Head<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Head {
            state: self.state.copied(),
            symbol: *self.symbol,
        }
    }
}

impl<'a, Q, S> Head<&'a mut Q, &'a mut S> {
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

    pub fn copied(&self) -> Head<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Head {
            state: self.state.copied(),
            symbol: *self.symbol,
        }
    }
}

impl<Q, S> From<(Q, S)> for Head<Q, S> {
    fn from((state, symbol): (Q, S)) -> Self {
        Self::new(State(state), symbol)
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
