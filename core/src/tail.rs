/*
    Appellation: tail <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

mod impl_tail;

#[allow(deprecated)]
mod impl_deprecated;

use crate::Direction;
use crate::Head;
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
pub struct Tail<Q, A> {
    pub direction: Direction,
    #[cfg_attr(feature = "serde", serde(alias = "state"))]
    pub next_state: State<Q>,
    #[cfg_attr(feature = "serde", serde(alias = "symbol", alias = "next_symbol"))]
    pub write_symbol: A,
}

impl<Q, A> Tail<Q, A>
where
    Q: RawState,
{
    pub const fn new(direction: Direction, next_state: Q, write_symbol: A) -> Self {
        Self {
            direction,
            next_state: State(next_state),
            write_symbol,
        }
    }
    /// returns a new instance of the [`Tail`] using the given direction and head
    pub fn from_head(direction: Direction, head: Head<Q, A>) -> Self {
        Self {
            direction,
            next_state: head.state,
            write_symbol: head.symbol,
        }
    }
    /// returns the direction, state, and symbol as a 3-tuple
    pub const fn as_tuple(&self) -> (Direction, &State<Q>, &A) {
        (self.direction, &self.next_state, &self.write_symbol)
    }
    /// consumes the tail and returns the direction, state, and symbol as a 3-tuple
    pub fn into_tuple(self) -> (Direction, State<Q>, A) {
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
    pub const fn symbol(&self) -> &A {
        &self.write_symbol
    }
    /// returns a mutable reference to the symbol of the tail
    pub const fn symbol_mut(&mut self) -> &mut A {
        &mut self.write_symbol
    }
    /// update the direction of the tail
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
    /// update the configured state for the tail
    pub fn set_state(&mut self, state: Q) {
        self.next_state = State(state);
    }
    /// update the defined symbol for the tail
    pub fn set_symbol(&mut self, symbol: A) {
        self.write_symbol = symbol;
    }
    /// consumes the current instance to create another with the given [`Direction`]
    pub fn with_direction(self, direction: Direction) -> Self {
        Self { direction, ..self }
    }
    /// consumes the current instance to create another with the given state
    pub fn with_state(self, state: Q) -> Self {
        Self {
            next_state: State(state),
            ..self
        }
    }
    /// Configures the tail with a new symbol
    pub fn with_symbol(self, symbol: A) -> Self {
        Self {
            write_symbol: symbol,
            ..self
        }
    }
    /// converts a [`Tail`] reference into an owned head.
    pub const fn as_head(&self) -> Head<&Q, &A> {
        Head {
            state: self.next_state.view(),
            symbol: &self.write_symbol,
        }
    }
    /// consumes the current tail to convert it into a [head](Head)
    pub fn into_head(self) -> Head<Q, A> {
        Head {
            state: self.next_state,
            symbol: self.write_symbol,
        }
    }
    /// returns an instance of the [head](Head) where each element within
    /// the created instance is a mutable reference
    pub const fn view(&self) -> Tail<&Q, &A> {
        Tail {
            direction: self.direction(),
            next_state: self.state().view(),
            write_symbol: self.symbol(),
        }
    }
    /// returns a new [`Tail`] containing mutabl references to the state and symbol
    pub const fn view_mut(&mut self) -> Tail<&mut Q, &mut A> {
        Tail {
            direction: self.direction,
            next_state: self.next_state.view_mut(),
            write_symbol: &mut self.write_symbol,
        }
    }
}
