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

#[doc(hidden)]
pub mod entry;
#[doc(hidden)]
pub mod iter;

pub(crate) mod prelude {
    pub use super::tape::Tape;
}

#[doc(hidden)]
pub trait RawTape {
    type Elem;

    fn as_slice(&self) -> &[Self::Elem];
}

/*
 ************* Implementations *************
*/
impl<T> RawTape for [T] {
    type Elem = T;

    fn as_slice(&self) -> &[Self::Elem] {
        &self
    }
}

impl<T> RawTape for Vec<T> {
    type Elem = T;

    fn as_slice(&self) -> &[Self::Elem] {
        &self
    }
}
