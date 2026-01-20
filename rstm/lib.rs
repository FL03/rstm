#![crate_name = "rstm"]
#![crate_type = "lib"]
//! # rstm
//!
//! Welcome to `rstm`, a flexible framework for creating and executing Turing machines (TM) in
//! Rust.
//!
//! ## Background
//!
//! Turing machines are abstract models of computation initially proposed by Alan Turing in
//! 1936. These models work by considering an infinite tape (memory) divided evenly into
//! uniform cells capable of storing a single value that is mutated by the head of the machine
//! according to some set of rules.
//!
//! ## Rules
//!
//! Rules or instructions for TMs can be defined as a 5-tuple consisting of:
//!
//! 1. Current State: The current state of the machine.
//! 2. Current Symbol: The symbol currently being read by the head.
//! 3. Next State: The state to transition to after executing the rule.
//! 4. Write Symbol: The symbol to write on the tape at the current head position.
//! 5. Shift: The direction to move the head (left, right, or stay).
//!
//! Here, we break the rule into two distinct components for clarity:
//!
//! - [`Head`]: defines the current state and symbol
//! - [`Tail`]: defines the shift direction, next state, and write symbol
//!
//! ## Basic Usage
//!
//! For more examples, please refer to the [`examples`](https://github.com/FL03/rstm/blob/main/rstm/examples) directory in the repository.
//!
//! ### `MovingHead` Example
//!
//! This example demonstrats how to initialize and run a Turing machine with a _moving head_.
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
