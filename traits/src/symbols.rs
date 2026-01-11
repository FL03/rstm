/*
    Appellation: symbols <module>
    Created At: 2026.01.11:13:55:56
    Contrib: @FL03
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
/// [`Symbolic`] is a marker trait used to signal a type that can be displayed.
pub trait Symbolic: core::fmt::Debug + core::fmt::Display {
    private! {}
}

/// The [`RawSymbol`] trait establishes the minimum requirements for a type to be used
/// as a symbol within a Turing machine.
pub trait RawSymbol: Symbolic + 'static {
    private! {}
}

/// The [`Symbol`] trait extends the [`RawSymbol`] to define the expected behaviors of a symbol
/// used within a Turing machine.
pub trait Symbol
where
    Self: RawSymbol
        + Clone
        + Copy
        + Default
        + Eq
        + Ord
        + PartialEq
        + PartialOrd
        + Send
        + Sync
        + core::hash::Hash,
{
}

/*
 ************* Implementations *************
*/

impl<S> Symbolic for S
where
    S: core::fmt::Debug + core::fmt::Display,
{
    seal! {}
}

impl<S> RawSymbol for S
where
    S: Symbolic + 'static,
{
    seal! {}
}

impl<S> Symbol for S where S: RawSymbol + Copy + Default + Eq + Ord + Send + Sync + core::hash::Hash {}

impl<S: Symbol> Alphabet for [S] {
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

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{Alphabet, Symbol};
    use alloc::vec::Vec;

    impl<S: Symbol> Alphabet for Vec<S> {
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
}