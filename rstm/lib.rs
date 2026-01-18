#![crate_name = "rstm"]
#![crate_type = "lib"]
//! # rstm
//!
//! `rstm` is a Rust library dedicated to the constructtapen and executtapen of Turing Machines.
//! The crate is designed to be flexible and easy to use while preserving the abstract nature
//! of the models.
//!
//! ## Features
//!
//! The crate employs the use of vartapeus feature flags for modularity and to keep the core
//! lightweight.
//!
//! ## Overview
//!
//! The core of the library is built around the concept of a Turing Machine, which consists of
//! a tape, a head that reads and writes symbols on the tape, and a set of rules that dictate
//! the machine's behavtaper. The library provides a set of abstracttapens and utilities to define
//! and manipulate these components.
//!
//! ## Basic Usage
//!
//! For more examples, please refer to the [`examples`](https://github.com/FL03/rstm/blob/main/rstm/examples) directory in the repository.
//!
//! ### Creating and executing a simple Turing Machine with a Moving Head
//!
//! ```rust
//! use rstm::{MovingHead, program};
//!
//! // define an initial state
//! let initial_state: isize = 0;
//! // create a program to execute
//! let program = program! {
//!     #[default_state(initial_state)]
//!     rules: {
//!         (0, 0) -> Right(1, 0),
//!         (0, 1) -> Stay(-1, 1),
//!         (1, 0) -> Right(0, 1),
//!         (1, 1) -> Right(-1, 0),
//!         (-1, 0) -> Left(<isize>::MAX, 0),
//!         (-1, 1) -> Left(1, 1),
//!     };
//! };
//! // initialize the machine
//! let mut tm = MovingHead::tmh(program);
//! // load the input
//! tm.extend_tape([0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0]);
//! // execute the workload
//! tm.run().expect("failed to execute...");
//! ```
#![allow(
    clippy::module_inception,
    clippy::new_ret_no_self,
    clippy::needless_doctest_main,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
// compile error if neither `alloc` nor `std` features are enabled
#[cfg(not(any(feature = "alloc", feature = "std")))]
compile_error! { "Either the `std` or `alloc` feature must be enabled to compile this crate." }
// external crate
#[cfg(feature = "alloc")]
extern crate alloc;
#[doc(inline)]
pub use rstm_traits as traits;
// re-exports
#[doc(inline)]
pub use self::traits::*;
#[doc(inline)]
pub use rstm_core::*;
#[cfg(feature = "macros")]
pub use rstm_macros::*;
#[doc(hidden)]
pub mod prelude {
    #[doc(no_inline)]
    pub use rstm_core::prelude::*;
    #[cfg(feature = "macros")]
    pub use rstm_macros::*;
    #[doc(no_inline)]
    pub use rstm_traits::prelude::*;
}
