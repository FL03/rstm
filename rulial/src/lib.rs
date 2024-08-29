/*
    Appellation: rstm-rulial <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Rulial
//!
//!
#[doc(inline)]
pub use self::space::BasePoint;

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
pub(crate) mod macros;
#[macro_use]
pub(crate) mod seal;

pub mod space;

pub mod prelude {
    pub use super::space::prelude::*;
}
