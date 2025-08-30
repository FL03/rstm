/*
    Appellation: tail <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Head;
use rstm_core::Direction;
use rstm_state::{RawState, State};

/// The [Tail] is a 3-tuple containing the direction, state, and symbol that an actor is
/// instructed to execute whenever it assumes the head configuration assigned to the tail.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[repr(C)]
pub struct Tail<Q, S> {
    pub direction: Direction,
    #[cfg_attr(feature = "serde", serde(alias = "state"))]
    pub next_state: State<Q>,
    #[cfg_attr(feature = "serde", serde(alias = "symbol", alias = "next_symbol"))]
    pub write_symbol: S,
}

impl<Q, S> Tail<Q, S>
where
    Q: RawState,
{
    pub const fn new(direction: Direction, next_state: State<Q>, write_symbol: S) -> Self {
        Self {
            direction,
            next_state,
            write_symbol,
        }
    }
    /// returns the direction, state, and symbol as a 3-tuple
    pub const fn as_tuple(&self) -> (Direction, &State<Q>, &S) {
        (self.direction, &self.next_state, &self.write_symbol)
    }
    /// consumes the tail and returns the direction, state, and symbol as a 3-tuple
    pub fn into_tuple(self) -> (Direction, State<Q>, S) {
        (self.direction, self.next_state, self.write_symbol)
    }
    /// returns the direction the [head](StdHead) is instructed to move
    pub const fn direction(&self) -> Direction {
        self.direction
    }
    /// returns the next state with an immutable reference to the inner value
    pub const fn state(&self) -> &State<Q> {
        &self.next_state
    }
    /// returns the next state with a mutable reference to the inner value
    pub const fn state_mut(&mut self) -> &mut State<Q> {
        &mut self.next_state
    }
    /// returns the symbol the [head](Head) is instructed to write
    pub const fn symbol(&self) -> &S {
        &self.write_symbol
    }
    /// returns a mutable reference to the symbol of the tail
    pub const fn symbol_mut(&mut self) -> &mut S {
        &mut self.write_symbol
    }
    /// updates the current [direction](Direction) and returns a mutable reference to the tail
    pub fn set_direction(&mut self, direction: Direction) -> &mut Self {
        self.direction = direction;
        self
    }
    /// updates the current [state](State) and returns a mutable reference to the tail
    pub fn set_state(&mut self, State(state): State<Q>) -> &mut Self {
        self.next_state = State(state);
        self
    }
    /// updates the current symbol and returns a mutable reference to the tail
    pub fn set_symbol(&mut self, symbol: S) -> &mut Self {
        self.write_symbol = symbol;
        self
    }
    /// Configures the tail with a new direction
    pub fn with_direction(self, direction: Direction) -> Self {
        Self { direction, ..self }
    }
    /// Configures the tail with a new state
    pub fn with_state(self, State(state): State<Q>) -> Self {
        Self {
            next_state: State(state),
            ..self
        }
    }
    /// Configures the tail with a new symbol
    pub fn with_symbol(self, symbol: S) -> Self {
        Self {
            write_symbol: symbol,
            ..self
        }
    }
    /// converts a [`Tail`] reference into an owned head.
    pub const fn as_head(&self) -> Head<&Q, &S> {
        Head {
            state: self.next_state.view(),
            symbol: &self.write_symbol,
        }
    }
    /// consumes the current tail to convert it into a [head](Head)
    pub fn into_head(self) -> Head<Q, S> {
        Head {
            state: self.next_state,
            symbol: self.write_symbol,
        }
    }
    /// returns an instance of the [head](Head) where each element within
    /// the created instance is a mutable reference
    pub const fn view(&self) -> Tail<&Q, &S> {
        Tail {
            direction: self.direction(),
            next_state: self.state().view(),
            write_symbol: self.symbol(),
        }
    }
    /// returns a new [`Tail`] containing mutabl references to the state and symbol
    pub fn view_mut(&mut self) -> Tail<&mut Q, &mut S> {
        Tail {
            direction: self.direction,
            next_state: self.next_state.view_mut(),
            write_symbol: &mut self.write_symbol,
        }
    }
    #[deprecated(
        since = "0.0.7",
        note = "use `view` instead, as it is more idiomatic and clearer."
    )]
    pub fn to_ref(&self) -> Tail<&Q, &S> {
        self.view()
    }
    #[deprecated(
        since = "0.0.7",
        note = "use `view_mut` instead, as it is more idiomatic and clearer."
    )]
    pub fn to_mut(&mut self) -> Tail<&mut Q, &mut S> {
        self.view_mut()
    }
}

impl<'a, Q, S> Tail<&'a Q, &'a S>
where
    Q: RawState,
{
    /// returns a new [`Tail`] with cloned elements
    pub fn cloned(&self) -> Tail<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Tail {
            direction: self.direction,
            next_state: self.next_state.cloned(),
            write_symbol: self.write_symbol.clone(),
        }
    }
    /// returns a new [`Tail`] with copied elements
    pub fn copied(&self) -> Tail<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Tail {
            direction: self.direction,
            next_state: self.next_state.copied(),
            write_symbol: *self.write_symbol,
        }
    }
}

impl<'a, Q, S> Tail<&'a mut Q, &'a mut S>
where
    Q: RawState,
{
    /// returns a new [`Tail`] with cloned elements
    pub fn cloned(&self) -> Tail<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Tail {
            direction: self.direction,
            next_state: self.next_state.cloned(),
            write_symbol: self.write_symbol.clone(),
        }
    }
    /// returns a new [`Tail`] with copied elements
    pub fn copied(&self) -> Tail<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Tail {
            direction: self.direction,
            next_state: self.next_state.copied(),
            write_symbol: *self.write_symbol,
        }
    }
}

impl<Q, S> core::fmt::Debug for Tail<Q, S>
where
    Q: RawState,
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Tail")
            .field(&self.direction)
            .field(&self.next_state)
            .field(&self.write_symbol)
            .finish()
    }
}

impl<Q, S> core::fmt::Display for Tail<Q, S>
where
    Q: RawState,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ direction: {}, state: {:?}, symbol: {} }}",
            self.direction, self.next_state, self.write_symbol
        )
    }
}
