/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::symbolic::Symbolic;

#[doc(hidden)]
pub mod actor;
pub mod symbolic;

pub(crate) mod prelude {
    pub use super::symbolic::*;
}

pub trait Shift<T> {
    type Output;

    fn shift(&self, tape: &T) -> Self::Output;
}
