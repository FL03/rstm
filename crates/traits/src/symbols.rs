/*
    Appellation: symbols <module>
    Created At: 2026.01.11:13:55:56
    Contrib: @FL03
*/
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// The [`Symbolic`] trait is a marker trait used to define types that can be used as symbols in
/// formal languages.
pub trait Symbolic
where
    Self: 'static
        + Clone
        + Default
        + Eq
        + PartialOrd
        + Send
        + Sync
        + core::fmt::Debug
        + core::fmt::Display
        + core::hash::Hash,
{
    private! {}
}
/// [`Alphabet`] describes a finite set of symbols used to construct a formal language.
///
/// Ideally, the alphabet should be implemented on unit enums since each symbol can be
/// represented as a unique variant and assigned a particular value. The values of the variants
/// may then be used as pointers, specifiying the location of the symbol w.r.t. the alphabet.
pub trait Alphabet {
    type Elem: Symbolic;

    fn as_slice(&self) -> &[Self::Elem];

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }
    #[cfg(feature = "alloc")]
    /// returns the alphabet as a vector
    fn to_vec(&self) -> Vec<Self::Elem>;
}

pub trait AlphabetMut: Alphabet {
    fn as_mut_slice(&mut self) -> &mut [Self::Elem];
}

/*
 ************* Implementations *************
*/

impl<S> Symbolic for S
where
    S: 'static
        + Clone
        + Default
        + Eq
        + Ord
        + Send
        + Sync
        + core::fmt::Debug
        + core::fmt::Display
        + core::hash::Hash,
{
    seal! {}
}

impl<S> Alphabet for [S]
where
    S: Symbolic,
{
    type Elem = S;

    fn as_slice(&self) -> &[S] {
        self
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    #[cfg(feature = "alloc")]
    fn to_vec(&self) -> Vec<S> {
        self.to_vec()
    }
}

impl<A> AlphabetMut for [A]
where
    A: Symbolic,
{
    fn as_mut_slice(&mut self) -> &mut [A] {
        &mut self[..]
    }
}

impl<S> Alphabet for &[S]
where
    S: Symbolic,
{
    type Elem = S;

    fn as_slice(&self) -> &[S] {
        self
    }

    fn is_empty(&self) -> bool {
        <[S]>::is_empty(self)
    }

    fn len(&self) -> usize {
        <[S]>::len(self)
    }

    #[cfg(feature = "alloc")]
    fn to_vec(&self) -> Vec<S> {
        <[S]>::to_vec(self)
    }
}

impl<S> Alphabet for &mut [S]
where
    S: Symbolic,
{
    type Elem = S;

    fn as_slice(&self) -> &[S] {
        self
    }

    fn is_empty(&self) -> bool {
        <[S]>::is_empty(self)
    }

    fn len(&self) -> usize {
        <[S]>::len(self)
    }

    #[cfg(feature = "alloc")]
    fn to_vec(&self) -> Vec<S> {
        <[S]>::to_vec(self)
    }
}

impl<S> AlphabetMut for &mut [S]
where
    S: Symbolic,
{
    fn as_mut_slice(&mut self) -> &mut [S] {
        self
    }
}

impl<const N: usize, S> Alphabet for [S; N]
where
    S: Symbolic,
{
    type Elem = S;

    fn as_slice(&self) -> &[S] {
        self
    }

    fn is_empty(&self) -> bool {
        <[S]>::is_empty(self)
    }

    fn len(&self) -> usize {
        <[S]>::len(self)
    }

    #[cfg(feature = "alloc")]
    fn to_vec(&self) -> Vec<S> {
        <[S]>::to_vec(self)
    }
}

impl<const N: usize, S> AlphabetMut for [S; N]
where
    S: Symbolic,
{
    fn as_mut_slice(&mut self) -> &mut [S] {
        &mut self[..]
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{Alphabet, AlphabetMut, Symbolic};
    use alloc::vec::Vec;

    impl<S> Alphabet for Vec<S>
    where
        S: Symbolic,
    {
        type Elem = S;

        fn as_slice(&self) -> &[S] {
            self.as_slice()
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

    impl<S> AlphabetMut for Vec<S>
    where
        S: Symbolic,
    {
        fn as_mut_slice(&mut self) -> &mut [S] {
            self.as_mut_slice()
        }
    }
}
