/*
    Appellation: tail <module>
    Created At: 2025.12.15:21:29:04
    Contrib: @FL03
*/
mod impl_tail;
mod impl_tail_ext;
mod impl_tail_repr;

use crate::Direction;
use crate::state::State;

/// A type alias for a [`Tail`] containing immutable references to the next state and symbol.
pub type TailRef<'a, Q, A> = Tail<&'a Q, &'a A>;
/// A type alias for a [`Tail`] containing mutable references to the next state and symbol.
pub type TailMut<'a, Q, A> = Tail<&'a mut Q, &'a mut A>;

/// [`RawTail`] is a sealed marker trait used to denote objects capable of being used to
/// represent the _tail_ of a rule for a Turing machine.
pub trait RawTail<Q, A> {
    private! {}
    /// returns the direction of the tail.
    fn direction(&self) -> Direction;
    /// returns an immutable reference to the next state.
    fn next_state(&self) -> &Q;
    /// returns a reference to the symbol configured for the head to write next.
    fn write_symbol(&self) -> &A;
}

/// The [`Tail`] of a rule defines the _reaction_ of the actor under specific conditions.
/// Specifically, it defines the next state, the symbol to write, and the direction to move
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct Tail<Q, A> {
    pub direction: Direction,
    #[cfg_attr(feature = "serde", serde(alias = "write_state"))]
    pub next_state: State<Q>,
    #[cfg_attr(feature = "serde", serde(alias = "next_symbol"))]
    pub write_symbol: A,
}

/*
 ************* Implementations *************
*/
impl<Q, A> RawTail<Q, A> for (Direction, State<Q>, A) {
    seal! {}
    /// returns the direction of the tail.
    fn direction(&self) -> Direction {
        self.0
    }
    /// returns an immutable reference to the next state.
    fn next_state(&self) -> &Q {
        &self.1
    }
    /// returns an immutable reference to the symbol to write.
    fn write_symbol(&self) -> &A {
        &self.2
    }
}

impl<Q, A> RawTail<Q, A> for Tail<Q, A> {
    seal! {}
    /// returns the direction of the tail.
    fn direction(&self) -> Direction {
        self.direction
    }
    /// returns an immutable reference to the next state.
    fn next_state(&self) -> &Q {
        self.next_state.get()
    }
    /// returns an immutable reference to the symbol to write.
    fn write_symbol(&self) -> &A {
        &self.write_symbol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tail_init() {
        let tail: Tail<&str, char> = Tail::new(Direction::Right, "q1", 'a');
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
