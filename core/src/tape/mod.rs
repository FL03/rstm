/*
    Appellation: tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Tape
//!
//! Idealized Turing machines consider a tape, or memory, that is infinite in both directions.
//! This tape is a one-dimensional array of symbols manipulated by the tape head according to
//! some set of pre-defined rules.
pub use self::tape::StdTape;

pub(crate) mod tape;

#[doc(hidden)]
pub mod entry;
#[doc(hidden)]
pub mod iter;
#[doc(hidden)]
pub mod slider;

pub(crate) mod prelude {
    pub use super::tape::StdTape;
}

#[doc(hidden)]
pub trait RawTape {
    type Elem;

    fn as_slice(&self) -> &[Self::Elem];
}

#[doc(hidden)]
pub trait Tape {}
