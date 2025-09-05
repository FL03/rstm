/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
mod impl_head;
mod impl_head_repr;

#[allow(deprecated)]
mod impl_deprecated;

use rstm_state::{RawState, State};

/// a type alias for a [`Head`] containing immutable references to its state and symbol
pub type HeadRef<'a, Q, S> = Head<&'a Q, &'a S>;
/// a type alias for a [`Head`] containing mutable references to its state and symbol
pub type HeadMut<'a, Q, S> = Head<&'a mut Q, &'a mut S>;

/// Converts to a [`Head`] by reference.
pub trait AsHead<Q, A>
where
    Q: RawState,
{
    fn as_head(&self) -> Head<Q, A>;
}
/// Consumes the caller to convert it into a [`Head`].
pub trait IntoHead<Q, A>
where
    Q: RawState,
{
    fn into_head(self) -> Head<Q, A>;
}

/// The [`Head`] of a Turing machine is defined to be a two-tuple consisting of a state and a
/// symbol. Our implementation is generic over both the state and symbol types, allowing for
/// flexibility in their representation(s).
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
#[repr(C)]
pub struct Head<Q, S> {
    #[cfg_attr(feature = "serde", serde(alias = "currentState"))]
    pub state: State<Q>,
    #[cfg_attr(feature = "serde", serde(alias = "currentSymbol"))]
    pub symbol: S,
}

/*
 ************* Implementations *************
*/

impl<Q, A, T> IntoHead<Q, A> for T
where
    Q: RawState,
    T: Into<Head<Q, A>>,
{
    fn into_head(self) -> Head<Q, A> {
        self.into()
    }
}

impl<Q, S> From<(Q, S)> for Head<Q, S>
where
    Q: RawState,
{
    fn from((state, symbol): (Q, S)) -> Self {
        Self::new(state, symbol)
    }
}

impl<Q, S> From<(State<Q>, S)> for Head<Q, S>
where
    Q: RawState,
{
    fn from((state, symbol): (State<Q>, S)) -> Self {
        Head { state, symbol }
    }
}

impl<Q, S> From<Head<Q, S>> for (State<Q>, S)
where
    Q: RawState,
{
    fn from(Head { state, symbol }: Head<Q, S>) -> Self {
        (state, symbol)
    }
}
