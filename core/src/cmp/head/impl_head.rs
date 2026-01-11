/*
    Appellation: impl_head <module>
    Created At: 2025.08.31:00:00:55
    Contrib: @FL03
*/
use super::{Head, HeadMut, HeadRef};
use crate::motion::HeadStep;
use crate::rule::Rule;
use crate::state::State;
use crate::tail::Tail;

/// The core implementation of the [`Head`] providing fundamental methods for its manipulation,
/// including various constructors, accessors, and mutators as well as convienience methods for
/// converting between representations.
impl<Q, A> Head<Q, A> {
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
    /// create an instance of the [`Head`] from a given [`Tail`]
    pub fn from_tail(
        Tail {
            next_state,
            write_symbol,
            ..
        }: Tail<Q, A>,
    ) -> Self {
        Head {
            state: next_state,
            symbol: write_symbol,
        }
    }
    /// Create a new instance from a 2-tuple: `(state, symbol)`
    pub fn from_tuple((state, symbol): (State<Q>, A)) -> Self {
        Self { state, symbol }
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
    #[inline]
    pub fn set_state(&mut self, state: Q) {
        self.state_mut().set(state)
    }
    /// updates the current symbol
    #[inline]
    pub fn set_symbol(&mut self, symbol: A) {
        self.symbol = symbol;
    }
    /// consumes the current instance to create another instance with the given state
    #[inline]
    pub fn with_state<Q2>(self, state: Q2) -> Head<Q2, A> {
        Head {
            state: State(state),
            symbol: self.symbol,
        }
    }
    /// consumes the current instance to create another instance with the given symbol
    #[inline]
    pub fn with_symbol<A2>(self, symbol: A2) -> Head<Q, A2> {
        Head {
            state: self.state,
            symbol,
        }
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
    /// [`swap`](core::mem::swap) the state and symbol of the current instance with another
    pub const fn swap(&mut self, other: &mut Self) {
        self.swap_state(other.state_mut());
        self.swap_symbol(other.symbol_mut());
    }
    /// [`swap`](core::mem::swap) the current state with another
    pub const fn swap_state(&mut self, other: &mut State<Q>) {
        core::mem::swap(self.state_mut(), other)
    }
    /// [`swap`](core::mem::swap) the current symbol with another
    pub const fn swap_symbol(&mut self, other: &mut A) {
        core::mem::swap(self.symbol_mut(), other)
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
    /// associates the given tail with the current head, returning a new [`Rule`]
    pub fn append(self, tail: Tail<Q, A>) -> Rule<Q, A> {
        Rule { head: self, tail }
    }
    /// tries reading the given tape using the head as its coordinates.
    pub fn read<T, U>(self, tape: &'_ [T]) -> Option<&U>
    where
        A: core::slice::SliceIndex<[T], Output = U>,
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
    /// use the given [`Tail`] to execute a single step using the [`HeadStep`] executor to 
    /// manage the process.
    /// 
    /// **Note**: this method is _lazy_, meaning that the actual step is not performed until 
    /// a valid operation is invoked on the executor.
    pub fn move_head<'a>(&'a mut self, tail: Tail<Q, A>) -> HeadStep<'a, Q, A> {
        HeadStep::new(self, tail)
    }
}

impl<Q, S> From<S> for Head<Q, S>
where
    Q: Default,
{
    fn from(symbol: S) -> Self {
        Self::new(Q::default(), symbol)
    }
}

impl<Q, S> From<(Q, S)> for Head<Q, S> {
    fn from((state, symbol): (Q, S)) -> Self {
        Self::new(state, symbol)
    }
}

impl<Q, S> From<(State<Q>, S)> for Head<Q, S> {
    fn from((state, symbol): (State<Q>, S)) -> Self {
        Head { state, symbol }
    }
}

impl<Q, S> From<Head<Q, S>> for (State<Q>, S) {
    fn from(Head { state, symbol }: Head<Q, S>) -> Self {
        (state, symbol)
    }
}
