/*
    Appellation: tail <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! The [`Tail`] of a rule defines the _reaction_ of the actor under specific conditions.
//! Specifically, it defines the next state, the symbol to write, and the direction to move
mod impl_tail;
mod impl_tail_repr;

#[allow(deprecated)]
mod impl_deprecated;

use crate::Direction;
use rstm_state::{RawState, State};

/// A type alias for a [`Tail`] containing immutable references to the next state and symbol.
pub type TailRef<'a, Q, A> = Tail<&'a Q, &'a A>;
/// A type alias for a [`Tail`] containing mutable references to the next state and symbol.
pub type TailMut<'a, Q, A> = Tail<&'a mut Q, &'a mut A>;

/// Converts a type into a [`Tail`] by reference.
pub trait AsTail<Q, A>
where
    Q: RawState,
{
    fn as_tail(&self) -> Tail<Q, A>;

    private! {}
}
/// A consuming trait for converting a type into a [`Tail`].
pub trait IntoTail<Q, A>
where
    Q: RawState,
{
    fn into_tail(self) -> Tail<Q, A>;

    private! {}
}

/// The [`Tail`] of a rule in a Turing machine
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct Tail<Q, A> {
    pub direction: Direction,
    #[cfg_attr(feature = "serde", serde(alias = "writeState"))]
    pub next_state: State<Q>,
    #[cfg_attr(feature = "serde", serde(alias = "nextSymbol"))]
    pub write_symbol: A,
}

/*
 ************* Implementations *************
*/

impl<Q, A, T> IntoTail<Q, A> for T
where
    Q: RawState,
    T: Into<Tail<Q, A>>,
{
    fn into_tail(self) -> Tail<Q, A> {
        self.into()
    }

    seal! {}
}

impl<Q, A> From<(Direction, State<Q>, A)> for Tail<Q, A>
where
    Q: RawState,
{
    fn from((direction, next_state, write_symbol): (Direction, State<Q>, A)) -> Self {
        Tail {
            direction,
            next_state,
            write_symbol,
        }
    }
}

impl<Q, A> From<(Direction, Q, A)> for Tail<Q, A>
where
    Q: RawState,
{
    fn from((direction, next_state, write_symbol): (Direction, Q, A)) -> Self {
        Tail {
            direction,
            next_state: State(next_state),
            write_symbol,
        }
    }
}

impl<Q, A> From<Tail<Q, A>> for (Direction, State<Q>, A)
where
    Q: RawState,
{
    fn from(tail: Tail<Q, A>) -> Self {
        (tail.direction, tail.next_state, tail.write_symbol)
    }
}
