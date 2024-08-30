/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Actors
//!
//! An actor describes an abstract model of computation that may find the solution to any
//! computable sequence, or algorithm.
#[doc(inline)]
pub use self::{actor::Actor, exec::Executor};

pub(crate) mod actor;
pub(crate) mod exec;

#[doc(hidden)]
pub mod engine;

pub(crate) mod prelude {
    pub use super::actor::Actor;
    pub use super::exec::Executor;
}

use crate::{rules::Program, Alphabet};

#[doc(hidden)]
pub trait Model {
    type Alpha: Alphabet;
}

#[doc(hidden)]
pub trait Runtime<Q, S> {
    fn load(&mut self, program: Program<Q, S>);

    fn run(&mut self);
}
