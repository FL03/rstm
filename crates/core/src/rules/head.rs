/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use rstm_state::{RawState, State};

/// a type alias for a [`Head`] containing immutable references to its state and symbol
pub type HeadRef<'a, Q, S> = Head<&'a Q, &'a S>;
/// a type alias for a [`Head`] containing mutable references to its state and symbol
pub type HeadMut<'a, Q, S> = Head<&'a mut Q, &'a mut S>;
/// a type alias for a [`Head`] containing a reference to the state and a mutable reference to
/// the symbol
pub type HeadEntry<'a, Q, S> = Head<&'a Q, &'a mut S>;
/// The [`Head`] of a Turing machine is defined to be a two-tuple consisting of a state and a
/// symbol. Our implementation is generic over both the state and symbol types, allowing for
/// flexibility in their representation(s).
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
#[repr(C)]
pub struct Head<Q, A> {
    #[cfg_attr(feature = "serde", serde(alias = "current_state"))]
    pub state: State<Q>,
    #[cfg_attr(feature = "serde", serde(alias = "current_symbol"))]
    pub symbol: A,
}
/// [`RawHead`] is a marker trait used to define an interface for objects capable of being
/// used as a _head_ in the context of Turing machine rules.
pub trait RawHead {
    type State: RawState;
    type Symbol;
    /// returns a reference to the current state of the head
    fn state(&self) -> &State<Self::State>;
    /// returns a reference to the symbol of the head
    fn symbol(&self) -> &Self::Symbol;

    private! {}
}
/// The [`RawHeadMut`] trait extends the [`RawHead`] trait by providing mutable access to
pub trait RawHeadMut: RawHead {
    /// returns a mutable reference to the current state of the head
    fn state_mut(&mut self) -> &mut State<Self::State>;
    /// returns a mutable reference to the symbol of the head
    fn symbol_mut(&mut self) -> &mut Self::Symbol;
}
/// The [`HeadRepr`] trait extends the [`RawHead`] trait with standard initialization routines.
pub trait HeadRepr: RawHead + Sized {
    /// creates a new head from the given state and symbol
    fn new(state: Self::State, symbol: Self::Symbol) -> Self;
    /// create a new head from the given state, using a default symbol
    fn from_state(state: Self::State) -> Self
    where
        Self::Symbol: Default,
    {
        Self::new(state, Default::default())
    }
    /// create a new head from the given symbol, using a default state
    fn from_symbol(symbol: Self::Symbol) -> Self
    where
        Self::State: Default,
    {
        Self::new(Default::default(), symbol)
    }
}

/*
 ************* Implementations *************
*/
impl<Q, A> RawHead for (State<Q>, A)
where
    Q: RawState,
{
    type State = Q;
    type Symbol = A;
    seal! {}
    /// returns an immutable reference to the state.
    fn state(&self) -> &State<Q> {
        &self.0
    }
    /// returns an immutable reference to the symbol.
    fn symbol(&self) -> &A {
        &self.1
    }
}

impl<Q, A> RawHeadMut for (State<Q>, A)
where
    Q: RawState,
{
    /// returns a mutable reference to the state.
    fn state_mut(&mut self) -> &mut State<Q> {
        &mut self.0
    }
    /// returns a mutable reference to the symbol.
    fn symbol_mut(&mut self) -> &mut A {
        &mut self.1
    }
}

impl<Q, A> HeadRepr for (State<Q>, A)
where
    Q: RawState,
{
    /// creates a new head from the given state and symbol
    fn new(state: Q, symbol: A) -> Self {
        (State(state), symbol)
    }
}

impl<Q, A> RawHead for Head<Q, A>
where
    Q: RawState,
{
    type State = Q;
    type Symbol = A;
    seal! {}
    /// returns an immutable reference to the state.
    fn state(&self) -> &State<Q> {
        &self.state
    }
    /// returns an immutable reference to the symbol.
    fn symbol(&self) -> &A {
        &self.symbol
    }
}

impl<Q, A> RawHeadMut for Head<Q, A>
where
    Q: RawState,
{
    /// returns a mutable reference to the state.
    fn state_mut(&mut self) -> &mut State<Q> {
        &mut self.state
    }
    /// returns a mutable reference to the symbol.
    fn symbol_mut(&mut self) -> &mut A {
        &mut self.symbol
    }
}

impl<Q, A> HeadRepr for Head<Q, A>
where
    Q: RawState,
{
    /// creates a new head from the given state and symbol
    fn new(state: Q, symbol: A) -> Self {
        Self {
            state: State(state),
            symbol,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Head;

    #[test]
    fn test_head_creation() {
        let head = Head::new("s", 0usize);
        assert_eq! { head, ("s", 0usize) }
    }
}
