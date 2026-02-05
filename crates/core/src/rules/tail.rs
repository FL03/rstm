/*
    Appellation: tail <module>
    Created At: 2025.12.15:21:29:04
    Contrib: @FL03
*/
use crate::Direction;
use rstm_state::{RawState, State};

/// A type alias for a [`Tail`] containing immutable references to the next state and symbol.
pub type TailRef<'a, Q, A> = Tail<&'a Q, &'a A>;
/// A type alias for a [`Tail`] containing mutable references to the next state and symbol.
pub type TailMut<'a, Q, A> = Tail<&'a mut Q, &'a mut A>;
/// The [`Tail`] of a rule defines the _reaction_ of the actor under specific conditions.
/// Specifically, it defines the next state, the symbol to write, and the direction to move
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
#[repr(C)]
pub struct Tail<Q, A> {
    /// defines the direction to move after writing the symbol
    #[cfg_attr(feature = "serde", serde(alias = "move_direction", alias = "dir"))]
    pub direction: Direction,
    #[cfg_attr(feature = "serde", serde(alias = "write_state"))]
    pub next_state: State<Q>,
    #[cfg_attr(feature = "serde", serde(alias = "next_symbol"))]
    pub write_symbol: A,
}
/// [`RawTail`] is a sealed marker trait used to denote objects capable of being used to
/// represent the _tail_ of a rule for a Turing machine.
pub trait RawTail {
    type State: RawState;
    type Symbol;
    private! {}
    /// returns the direction of the tail.
    fn direction(&self) -> Direction;
    /// returns an immutable reference to the next state.
    fn next_state(&self) -> &State<Self::State>;
    /// returns a reference to the symbol configured for the head to write next.
    fn write_symbol(&self) -> &Self::Symbol;
}
/// The [`RawTailMut`] provides mutable access to the components of a tail.
pub trait RawTailMut: RawTail {
    /// returns a mutable reference to the direction of the tail.
    fn direction_mut(&mut self) -> &mut Direction;
    /// returns a mutable reference to the next state.
    fn next_state_mut(&mut self) -> &mut State<Self::State>;
    /// returns a mutable reference to the symbol to write.
    fn write_symbol_mut(&mut self) -> &mut Self::Symbol;
}
/// The [`TailRepr`] trait extends the [`RawTail`] trait with standard initialization routines.
pub trait TailRepr: RawTail + Sized {
    /// creates a new tail from the given direction, next state, and symbol to write
    fn new(direction: Direction, next_state: Self::State, write_symbol: Self::Symbol) -> Self;
    /// creates a new tail that moves right after writing the symbol and transitioning to the
    /// next state
    fn right(next_state: Self::State, write_symbol: Self::Symbol) -> Self {
        Self::new(Direction::Right, next_state, write_symbol)
    }
    /// creates a new tail that moves left after writing the symbol and transitioning to the
    /// next state
    fn left(next_state: Self::State, write_symbol: Self::Symbol) -> Self {
        Self::new(Direction::Left, next_state, write_symbol)
    }
    fn stay(next_state: Self::State, write_symbol: Self::Symbol) -> Self {
        Self::new(Direction::Stay, next_state, write_symbol)
    }
}
/*
 ************* Implementations *************
*/
impl<Q, A> RawTail for (Direction, State<Q>, A)
where
    Q: RawState,
{
    type State = Q;
    type Symbol = A;

    seal! {}
    /// returns the direction of the tail.
    fn direction(&self) -> Direction {
        self.0
    }
    /// returns an immutable reference to the next state.
    fn next_state(&self) -> &State<Q> {
        &self.1
    }
    /// returns an immutable reference to the symbol to write.
    fn write_symbol(&self) -> &A {
        &self.2
    }
}

impl<Q, A> RawTailMut for (Direction, State<Q>, A)
where
    Q: RawState,
{
    /// returns a mutable reference to the direction of the tail.
    fn direction_mut(&mut self) -> &mut Direction {
        &mut self.0
    }
    /// returns a mutable reference to the next state.
    fn next_state_mut(&mut self) -> &mut State<Q> {
        &mut self.1
    }
    /// returns a mutable reference to the symbol to write.
    fn write_symbol_mut(&mut self) -> &mut A {
        &mut self.2
    }
}

impl<Q, A> TailRepr for (Direction, State<Q>, A)
where
    Q: RawState,
{
    /// creates a new tail from the given direction, next state, and symbol to write
    fn new(direction: Direction, next_state: Q, write_symbol: A) -> Self {
        (direction, State(next_state), write_symbol)
    }
}

impl<Q, A> RawTail for Tail<Q, A>
where
    Q: RawState,
{
    type State = Q;
    type Symbol = A;

    seal! {}
    /// returns the direction of the tail.
    fn direction(&self) -> Direction {
        self.direction
    }
    /// returns an immutable reference to the next state.
    fn next_state(&self) -> &State<Q> {
        &self.next_state
    }
    /// returns an immutable reference to the symbol to write.
    fn write_symbol(&self) -> &A {
        &self.write_symbol
    }
}
impl<Q, A> RawTailMut for Tail<Q, A>
where
    Q: RawState,
{
    /// returns a mutable reference to the direction of the tail.
    fn direction_mut(&mut self) -> &mut Direction {
        &mut self.direction
    }
    /// returns a mutable reference to the next state.
    fn next_state_mut(&mut self) -> &mut State<Q> {
        &mut self.next_state
    }
    /// returns a mutable reference to the symbol to write.
    fn write_symbol_mut(&mut self) -> &mut A {
        &mut self.write_symbol
    }
}

impl<Q, A> TailRepr for Tail<Q, A>
where
    Q: RawState,
{
    /// creates a new tail from the given direction, next state, and symbol to write
    fn new(direction: Direction, next_state: Q, write_symbol: A) -> Self {
        Self {
            direction,
            next_state: State(next_state),
            write_symbol,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tail_init() {
        let tail: Tail<&str, char> = Tail::right("q1", 'a');
        assert_eq! {
            tail,
            (
                Direction::Right,
                State("q1"),
                'a'
            )
        }
    }
}
