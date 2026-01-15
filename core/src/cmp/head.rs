/*
    Appellation: head <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
mod impl_head;
mod impl_head_ext;
mod impl_head_repr;

use rstm_state::State;

/// a type alias for a [`Head`] containing immutable references to its state and symbol
pub type HeadRef<'a, Q, S> = Head<&'a Q, &'a S>;
/// a type alias for a [`Head`] containing mutable references to its state and symbol
pub type HeadMut<'a, Q, S> = Head<&'a mut Q, &'a mut S>;
/// a type alias for a [`Head`] containing a reference to the state and a mutable reference to
/// the symbol
pub type HeadEntry<'a, Q, S> = Head<&'a Q, &'a mut S>;
/// [`RawHead`] is a marker trait used to define an interface for objects capable of being
/// used as a _head_ in the context of Turing machine rules.
pub trait RawHead<Q, S> {
    fn state(&self) -> &State<Q>;
    fn symbol(&self) -> &S;

    private! {}
}

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
pub struct Head<Q, S> {
    #[cfg_attr(feature = "serde", serde(alias = "current_state"))]
    pub state: State<Q>,
    #[cfg_attr(feature = "serde", serde(alias = "current_symbol"))]
    pub symbol: S,
}

/*
 ************* Implementations *************
*/
impl<Q, A> RawHead<Q, A> for (State<Q>, A) {
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

impl<Q, A> RawHead<Q, A> for Head<Q, A> {
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

#[cfg(test)]
mod tests {
    use super::Head;

    #[test]
    fn test_head_creation() {
        let head = Head::new("s", 0usize);
        assert_eq! { head, ("s", 0usize) }
    }
}
