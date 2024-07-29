/*
    Appellation: tail <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Direction, State, StdHead};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct StdTail<Q = String, S = char> {
    pub(crate) direction: Direction,
    pub(crate) state: State<Q>,
    pub(crate) symbol: S,
}

impl<Q, S> StdTail<Q, S> {
    pub fn new(direction: Direction, State(state): State<Q>, symbol: S) -> Self {
        Self {
            direction,
            state: State(state),
            symbol,
        }
    }
    /// Returns the direction, state, and symbol as a 3-tuple
    pub fn as_tuple(&self) -> (Direction, &State<Q>, &S) {
        (self.direction, &self.state, &self.symbol)
    }
    /// Returns an instance of the [head](StdHead) which owns the current state and symbol
    pub fn as_head(&self) -> StdHead<&'_ Q, &'_ S> {
        super::StdHead::new(self.state.to_ref(), &self.symbol)
    }
    /// Consumes the tail and returns the head
    pub fn into_head(self) -> StdHead<Q, S> {
        super::StdHead::new(self.state, self.symbol)
    }
    /// Consumes the tail and returns the direction, state, and symbol as a 3-tuple
    pub fn into_tuple(self) -> (Direction, State<Q>, S) {
        (self.direction, self.state, self.symbol)
    }
    /// Returns the direction the [head](StdHead) is instructed to move
    pub fn direction(&self) -> Direction {
        self.direction
    }
    /// Returns the next [state](State) the agent is instructed to move to
    pub fn next_state(&self) -> State<&'_ Q> {
        self.state.to_ref()
    }
    /// Returns the symbol the [head](StdHead) is instructed to write
    pub const fn write_symbol(&self) -> &S {
        &self.symbol
    }
}
