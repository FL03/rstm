/*
    Appellation: ops <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Operations
//!
//! Overloadable operations used throughout the `rstm` crate.
#[doc(inline)]
pub use self::transform::*;

#[doc(hidden)]
pub mod shift;
pub mod transform;

pub(crate) mod prelude {
    pub use super::transform::*;
}
