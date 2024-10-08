/*
    Appellation: shift <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::direction::*;

pub(crate) mod direction;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::direction::*;
}

/// [`Shift`] describes a generalized shift operation;
/// w.r.t. Turing machines, Moore (1990) describes a shift operation as a _movement_ of the
/// tape head
pub trait Shift<T> {
    type Output;

    fn shift(&self, step: T) -> Self::Output;
}

pub trait ApplyOnce<T, F> where F: FnOnce(T) -> Self::Output {
    type Output;

    fn apply(&self, f: F, args: T) -> F::Output;
}