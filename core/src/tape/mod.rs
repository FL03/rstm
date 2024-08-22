/*
    Appellation: tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Tape
//!
//! Idealized Turing machines consider a tape, or memory, that is infinite in both directions.
//! This tape is a one-dimensional array of symbols manipulated by the tape head according to
//! some set of pre-defined rules.
pub use self::tape::Tape;

pub(crate) mod tape;

pub(crate) mod prelude {
    pub use super::tape::Tape;
}

/// [RawTape] is a trait that provides a common interface for tape-like structures.
pub trait RawTape {
    type Elem;

    private!();

    fn as_slice(&self) -> &[Self::Elem];

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

/*
 ************* Implementations *************
*/
impl<T> RawTape for [T] {
    type Elem = T;

    seal!();

    fn as_slice(&self) -> &[Self::Elem] {
        &self
    }

    fn is_empty(&self) -> bool {
        <[T]>::is_empty(self)
    }

    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<T> RawTape for Vec<T> {
    type Elem = T;

    seal!();

    fn as_slice(&self) -> &[Self::Elem] {
        Vec::as_slice(self)
    }

    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }
}
