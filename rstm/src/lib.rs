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
//! ## Examples
//!
//! For more examples, please refer to the [`examples`](https://github.com/FL03/rstm/blob/main/rstm/examples) directory in the repository.
//!
//! ### Creating and executing a simple Turing Machine with a Moving Head
//!
//! ```rust
//!    use rstm::prelude::{Program, TMH};
//!
//!    fn main() -> rstm::Result<()> {
//!        // initialize the logger
//!        tracing_subscriber::fmt()
//!            .with_max_level(tracing::Level::DEBUG)
//!            .with_target(false)
//!            .with_timer(tracing_subscriber::fmt::time::uptime())
//!            .init();
//!        tracing::info!("Welcome to rstm!");
//!        // define some input for the machine
//!        let input = [0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0];
//!       // initialize the state of the machine
//!        let initial_state: isize = 0;
//!        // define the ruleset for the machine
//!        let program: Program<isize, usize> = rstm::program! {
//!            #[default_state(initial_state)]
//!            rules: {
//!                (0, 0) -> Right(1, 0);
//!                (0, 1) -> Left(-1, 1);
//!                (1, 0) -> Right(0, 1);
//!                (1, 1) -> Right(-1, 0);
//!                (-1, 0) -> Left(<isize>::MAX, 0);
//!                (-1, 1) -> Left(1, 1);
//!            };
//!        };
//!        // create a new instance of the machine
//!        let tm = TMH::new(initial_state, input.to_vec());
//!        // execute the program
//!        dbg!(tm).execute(program).run()?;
//!        Ok(())
//!    }
//! ```
#![allow(
    clippy::module_incepttapen,
    clippy::new_ret_no_self,
    clippy::needless_doctest_main,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]
#![crate_name = "rstm"]
#![crate_type = "lib"]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "macros")]
#[macro_use]
mod macros {
    #[macro_use]
    pub(crate) mod program;
    #[macro_use]
    pub(crate) mod rules;
}

#[cfg(feature = "actors")]
#[doc(inline)]
pub use rstm_actors as actors;
#[doc(inline)]
pub use rstm_core::*;
#[cfg(feature = "programs")]
#[doc(inline)]
pub use rstm_programs as programs;
#[cfg(feature = "tape")]
#[doc(inline)]
pub use rstm_tape as tape;

pub mod prelude {
    #[cfg(feature = "macros")]
    pub use crate::{rule, rules};
    #[cfg(feature = "actors")]
    pub use rstm_actors::prelude::*;
    #[doc(no_inline)]
    pub use rstm_core::prelude::*;
    #[cfg(feature = "programs")]
    pub use rstm_programs::prelude::*;
    #[cfg(feature = "tape")]
    pub use rstm_tape::prelude::*;
}
