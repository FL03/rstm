/*
    Appellation: mem <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Memory (mem)
//!
//!
#[doc(inline)]
pub use self::memory::*;

pub mod memory;
#[doc(hidden)]
pub mod store;
pub mod tape;

pub(crate) mod prelude {
    pub use super::tape::prelude::*;
}

pub(crate) type Cell<Q, S> = crate::Head<Q, S>;
