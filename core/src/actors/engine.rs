/*
    Appellation: engine <module>
    Created At: 2026.01.11:13:48:50
    Contrib: @FL03
*/
//! this modules provides the [`RawEngine`] and [`Engine`] traits alongside additional
//! implementations for experiementing and building Turing machine engines
#[cfg(feature = "alloc")]
#[doc(inline)]
pub use self::turing_engine::TuringEngine;

#[cfg(feature = "alloc")]
mod turing_engine;

use super::RawActor;

use crate::program::Program;
use rstm_state::RawState;

/// The [`RawEngine`] trait defines the basis for compatible engines within the system.
pub trait RawEngine<Q, A>
where
    Q: RawState,
{
    type Driver: RawActor;

    private!();
}

/// An [`Engine`] is responsible for processing input according to some set of pre-defined
/// rules using the configured driver.
pub trait Engine<Q, S>: RawEngine<Q, S>
where
    Q: RawState,
{
    fn load(&mut self, program: Program<Q, S>);

    fn run(&mut self) -> crate::Result<()>;
}
