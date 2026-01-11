/*
    Appellation: engine <module>
    Created At: 2026.01.11:13:51:13
    Contrib: @FL03
*/
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
