/*
    Appellation: symbolic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [Alphabet] describes a finite set of symbols used to construct a formal language.
///
/// Ideally, the alphabet should be implemented on unit enums since
/// each symbol can be represented as a unique variant and assigned
/// a particular value. The values of the variants may then be used
/// as pointers, specifiying the location of the symbol w.r.t. the
/// alphabet.
pub trait Alphabet {
    type Elem;

    fn as_slice(&self) -> &[Self::Elem];

    fn as_mut_slice(&mut self) -> &mut [Self::Elem];

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }

    fn to_vec(&self) -> Vec<Self::Elem>;
}

/// [Symbolic] is a trait denoting types that can be used as symbols;
/// this is useful for allowing symbols to represented with [char] or
/// be a position on the tape, value mapping for an alphabet,.
pub trait Symbolic
where
    Self: Clone
        + Copy
        + Eq
        + Ord
        + PartialEq
        + PartialOrd
        + core::fmt::Debug
        + core::fmt::Display
        + core::hash::Hash
        + Send
        + Sync
        + 'static,
{
}

/*
 ************* Implementations *************
*/
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

impl<S: Symbolic> Alphabet for [S] {
    type Elem = S;

    fn as_slice(&self) -> &[S] {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [S] {
        self
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn to_vec(&self) -> Vec<S> {
        self.to_vec()
    }
}

impl<S: Symbolic> Alphabet for Vec<S> {
    type Elem = S;

    fn as_slice(&self) -> &[S] {
        self.as_slice()
    }

    fn as_mut_slice(&mut self) -> &mut [S] {
        self.as_mut_slice()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn to_vec(&self) -> Vec<S> {
        self.clone()
    }
}

impl<S> Symbolic for S where
    S: Copy
        + Eq
        + Ord
        + core::fmt::Debug
        + core::fmt::Display
        + core::hash::Hash
        + Send
        + Sync
        + 'static
{
}
