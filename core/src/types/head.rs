/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::State;

/// The [Head] is formally defined to be a 2-tuple consisting of a state / symbol pair.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
pub struct Head<Q, S> {
    #[cfg_attr(feature = "serde", serde(alias = "current_state"))]
    pub state: State<Q>,
    #[cfg_attr(feature = "serde", serde(alias = "current_symbol"))]
    pub symbol: S,
}

impl<Q, S> Head<Q, S> {
    pub fn new(state: State<Q>, symbol: S) -> Self {
        Self { state, symbol }
    }
    /// Create a new instance of the [Head] using the given state and default symbol.
    pub fn from_state(state: State<Q>) -> Self
    where
        S: Default,
    {
        Self::new(state, S::default())
    }
    /// Create a new instance of the [Head] using the given symbol and default state.
    pub fn from_symbol(symbol: S) -> Self
    where
        Q: Default,
    {
        Self::new(State::default(), symbol)
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
    /// Replaces the current state and symbol with the given state and symbol; returns the
    /// previous instance of the head.
    pub fn replace(&mut self, state: State<Q>, symbol: S) -> Self {
        Head {
            state: core::mem::replace(&mut self.state, state),
            symbol: core::mem::replace(&mut self.symbol, symbol),
        }
    }
    /// Replaces the current state with the given state, returing the previous state
    pub fn replace_state(&mut self, state: State<Q>) -> State<Q> {
        core::mem::replace(&mut self.state, state)
    }
    /// Replaces the current symbol with the given symbol, returning the previous symbol
    pub fn replace_symbol(&mut self, symbol: S) -> S {
        core::mem::replace(&mut self.symbol, symbol)
    }

    pub fn swap(&mut self, other: &mut Self) {
        core::mem::swap(&mut self.state, &mut other.state);
        core::mem::swap(&mut self.symbol, &mut other.symbol);
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
            symbol: direction.apply_unsigned(self.symbol),
            ..self
        }
    }

    pub fn shift_inplace(&mut self, direction: crate::Direction) {
        self.symbol = direction.apply_unsigned(self.symbol);
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

impl<Q, S> core::fmt::Debug for Head<Q, S>
where
    Q: core::fmt::Debug,
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Head")
            .field(&self.state)
            .field(&self.symbol)
            .finish()
    }
}

impl<Q, S> core::fmt::Display for Head<Q, S>
where
    Q: core::fmt::Display,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {})", self.state, self.symbol)
    }
}

impl<Q, S> PartialEq<State<Q>> for Head<Q, S>
where
    Q: PartialEq,
{
    fn eq(&self, state: &State<Q>) -> bool {
        &self.state == state
    }
}

impl<Q, S> PartialEq<Head<Q, S>> for State<Q>
where
    Q: PartialEq,
{
    fn eq(&self, head: &Head<Q, S>) -> bool {
        *self == *head.state
    }
}

impl<'a, Q, S> PartialEq<Head<Q, S>> for State<&'a Q>
where
    Q: PartialEq,
{
    fn eq(&self, head: &Head<Q, S>) -> bool {
        *self == head.state()
    }
}

impl<'a, Q, S> PartialEq<State<&'a Q>> for Head<Q, S>
where
    Q: PartialEq,
{
    fn eq(&self, state: &State<&'a Q>) -> bool {
        self.state() == *state
    }
}

impl<Q, S> PartialEq<(State<Q>, S)> for Head<Q, S>
where
    Q: PartialEq,
    S: PartialEq,
{
    fn eq(&self, (state, symbol): &(State<Q>, S)) -> bool {
        &self.state == state && &self.symbol == symbol
    }
}

impl<Q, S> PartialEq<(Q, S)> for Head<Q, S>
where
    State<Q>: PartialEq,
    Q: PartialEq,
    S: PartialEq,
{
    fn eq(&self, (state, symbol): &(Q, S)) -> bool {
        &self.state == state && &self.symbol == symbol
    }
}

impl<Q, S> PartialEq<Head<Q, S>> for (State<Q>, S)
where
    Q: PartialEq,
    S: PartialEq,
{
    fn eq(&self, head: &Head<Q, S>) -> bool {
        head.state == self.0 && head.symbol == self.1
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
