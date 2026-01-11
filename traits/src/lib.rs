/*
    Appellation: rstmt-traits <library>
    Created At: 2026.01.11:10:44:09
    Contrib: @FL03
*/
//! traits used to define common behaviors, operations, and interfaces for Turing machines and
//! their components.
#![crate_type = "lib"]
#![allow(
    clippy::missing_saftey_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::should_implement_trait,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
// external crates
#[cfg(feature = "alloc")]
extern crate alloc;
// macros
#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub(crate) mod seal;
}
// modules
pub mod ops {
    //! useful operations for finite state machines (Turing Machines) and their rules
    #[doc(inline)]
    pub use self::{execute::*, increment::*, percentage::*};

    mod execute;
    mod increment;
    mod percentage;
}
// re-exports
#[doc(inline)]
pub use self::ops::*;
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use crate::ops::*;
}
