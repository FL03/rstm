/*
    Appellation: engine <module>
    Created At: 2025.08.30:00:15:55
    Contrib: @FL03
*/
use super::Handle;

use rstm_core::Tail;
use rstm_rules::{Program, Rule};
use rstm_state::RawState;

/// The [`RawEngine`] trait defines the basis for compatible engines within the system.
pub trait RawEngine<Q, A>
where
    Q: RawState,
{
    type Driver: super::driver::Driver;

    private!();
}

pub trait Loader<Q, S>
where
    Q: RawState,
{
    fn load<I>(&mut self, rules: I, default_state: Option<Q>)
    where
        I: IntoIterator<Item = Rule<Q, S>>;
}

/// An [`Engine`] is responsible for processing input according to some set of pre-defined
/// rules using the configured driver.
pub trait Engine<Q, S>: Handle<Tail<Q, S>>
where
    Q: RawState,
{
    fn load(&mut self, program: Program<Q, S>);

    fn run(&mut self) -> crate::Result<()>;
}
