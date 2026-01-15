/*
    Appellation: engine <module>
    Created At: 2026.01.11:13:48:50
    Contrib: @FL03
*/
//! this modules provides the [`RawEngine`] and [`Engine`] traits alongside additional
//! implementations for experiementing and building Turing machine engines

use super::RawDriver;

use crate::programs::Program;
use rstm_state::RawState;

/// The [`RawEngine`] trait defines the basis for compatible engines within the system.
pub trait RawEngine<Q, A>
where
    Q: RawState,
{
    type Driver: RawDriver<Q, A>;

    private!();
}

/// An [`Engine`] is responsible for processing input according to some set of pre-defined
/// rules using the configured driver.
pub trait Engine<Q, A>: RawEngine<Q, A>
where
    Q: RawState,
{
    fn load(&mut self, program: Program<Q, A>);

    fn run(&mut self) -> crate::Result<()>;
}
