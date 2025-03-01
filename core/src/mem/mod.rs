/*
    Appellation: mem <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Memory (mem)
//!
//!
#[doc(inline)]
pub use self::{memory::*, tape::StdTape};

mod memory;

#[doc(hidden)]
pub mod cell;
#[doc(hidden)]
pub mod snapshot;
#[doc(hidden)]
pub mod store;
pub mod tape;

pub(crate) mod prelude {
    pub use super::memory::{Memory, MemoryMut, RawMemory};
    pub use super::tape::prelude::*;
}

#[allow(dead_code)]
pub(crate) type Cell<Q, S> = crate::Head<Q, S>;
