/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{direction::Direction, head::Head, tail::Tail};

pub(crate) mod direction;
pub(crate) mod head;
pub(crate) mod tail;

#[doc(hidden)]
pub mod snapshot;

pub(crate) mod prelude {
    pub use super::direction::*;
    pub use super::head::Head;
    pub use super::tail::Tail;
}

/// A type alias for a [Result] with our custom error type: [`Error`](crate::Error)
pub type Result<T = ()> = core::result::Result<T, crate::Error>;
