/*
    Appellation: rstmt-traits <library>
    Created At: 2026.01.11:10:44:09
    Contrib: @FL03
*/
//! traits used to define common behaviors, operations, and interfaces for Turing machines and
//! their components.
#![crate_type = "lib"]
#![allow(
    clippy::len_without_is_empty,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::new_ret_no_self,
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
mod io;
mod symbols;

mod ops {
    //! useful operations for finite state machines (Turing Machines) and their rules
    #[doc(inline)]
    pub use self::{execute::*, handle::*, increment::*, percentage::*, step::*};

    mod execute;
    mod handle;
    mod increment;
    mod percentage;
    mod step;
}
// re-exports
#[doc(inline)]
pub use self::{io::*, ops::*, symbols::*};
// prelude
#[doc(hidden)]
pub mod prelude {
    pub use crate::io::*;
    pub use crate::ops::*;
    pub use crate::symbols::*;
}
