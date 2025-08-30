/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstm_core::Direction;
use rstm_state::{RawState, State};

/// The [Head] is formally defined to be a 2-tuple consisting of a state / symbol pair.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
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
    pub const fn new(state: State<Q>, symbol: S) -> Self {
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
    pub fn with_state(self, state: State<Q>) -> Self {
        Self { state, ..self }
    }
    /// Updates the current symbol and returns a new head
    pub fn with_symbol(self, symbol: S) -> Self {
        Self { symbol, ..self }
    }
    /// Returns a reference to the current state
    pub const fn state(&self) -> &State<Q> {
        &self.state
    }
    /// Returns a mutable reference to the current [State]
    pub const fn state_mut(&mut self) -> &mut State<Q> {
        &mut self.state
    }
    /// Returns a reference to the current symbol
    pub const fn symbol(&self) -> &S {
        &self.symbol
    }
    /// Returns a mutable reference to the current symbol
    pub const fn symbol_mut(&mut self) -> &mut S {
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
    pub fn set_state(&mut self, State(state): State<Q>) -> &mut Self {
        self.state_mut().set(state);
        self
    }
    /// Updates the current symbol
    pub fn set_symbol(&mut self, symbol: S) -> &mut Self {
        self.symbol = symbol;
        self
    }
    /// Replaces the current state and symbol with the given state and symbol; returns the
    /// previous instance of the head.
    pub fn replace(&mut self, state: State<Q>, symbol: S) -> Self {
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
    /// Updates the current [State] and symbol
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
    pub fn view_mut(&mut self) -> Head<&mut Q, &mut S> {
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
    #[deprecated(
        since = "0.0.7",
        note = "use `view` instead, as it is more idiomatic and clearer."
    )]
    pub fn to_ref(&self) -> Head<&Q, &S> {
        Head {
            state: self.state.view(),
            symbol: &self.symbol,
        }
    }
    #[deprecated(
        since = "0.0.7",
        note = "use `view_mut` instead, as it is more idiomatic and clearer."
    )]
    pub fn to_mut(&mut self) -> Head<&mut Q, &mut S> {
        self.view_mut()
    }
}

impl<'a, Q, S> Head<&'a Q, &'a S>
where
    Q: RawState,
{
    /// returns a new [`Head`] with cloned elements
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
    /// returns a new [`Head`] with copied elements
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

impl<'a, Q, S> Head<&'a mut Q, &'a mut S>
where
    Q: RawState,
{
    /// returns a new [`Head`] with cloned elements
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
    /// returns a new [`Head`] with copied elements
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

impl<Q> Head<Q, usize>
where
    Q: RawState,
{
    pub fn shift(self, direction: Direction) -> Self {
        Self {
            symbol: direction.apply_unsigned(self.symbol),
            ..self
        }
    }

    pub fn shift_inplace(&mut self, direction: Direction) {
        self.symbol = direction.apply_unsigned(self.symbol);
    }
}

impl<Q, S> core::fmt::Debug for Head<Q, S>
where
    Q: RawState,
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
    Q: RawState,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{{ state: {:?}, symbol: {} }}", self.state, self.symbol)
    }
}

impl<Q, S> PartialEq<State<Q>> for Head<Q, S>
where
    Q: RawState + PartialEq,
{
    fn eq(&self, state: &State<Q>) -> bool {
        self.state() == state
    }
}

impl<Q, S> PartialEq<Head<Q, S>> for State<Q>
where
    Q: RawState + PartialEq,
{
    fn eq(&self, head: &Head<Q, S>) -> bool {
        self == head.state()
    }
}

impl<Q, S> PartialEq<Head<Q, S>> for State<&Q>
where
    Q: RawState + PartialEq,
{
    fn eq(&self, head: &Head<Q, S>) -> bool {
        *self == head.state().view()
    }
}

impl<'a, Q, S> PartialEq<State<&'a Q>> for Head<Q, S>
where
    Q: RawState + PartialEq,
{
    fn eq(&self, state: &State<&'a Q>) -> bool {
        self.state().view() == *state
    }
}

impl<Q, S> PartialEq<(State<Q>, S)> for Head<Q, S>
where
    Q: RawState + PartialEq,
    S: PartialEq,
{
    fn eq(&self, (state, symbol): &(State<Q>, S)) -> bool {
        &self.state == state && &self.symbol == symbol
    }
}

impl<Q, S> PartialEq<(Q, S)> for Head<Q, S>
where
    State<Q>: PartialEq,
    Q: RawState + PartialEq,
    S: PartialEq,
{
    fn eq(&self, (state, symbol): &(Q, S)) -> bool {
        self.state() == state && self.symbol() == symbol
    }
}

impl<Q, S> PartialEq<Head<Q, S>> for (State<Q>, S)
where
    Q: RawState + PartialEq,
    S: PartialEq,
{
    fn eq(&self, head: &Head<Q, S>) -> bool {
        head.state() == &self.0 && head.symbol() == &self.1
    }
}

impl<Q, S> From<(Q, S)> for Head<Q, S>
where
    Q: RawState,
{
    fn from((state, symbol): (Q, S)) -> Self {
        Self::new(State(state), symbol)
    }
}

impl<Q, S> From<(State<Q>, S)> for Head<Q, S>
where
    Q: RawState,
{
    fn from((state, symbol): (State<Q>, S)) -> Self {
        Self::new(state, symbol)
    }
}

impl<Q, S> From<Head<Q, S>> for (State<Q>, S)
where
    Q: RawState,
{
    fn from(head: Head<Q, S>) -> Self {
        head.into_tuple()
    }
}
