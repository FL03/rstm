/*
    Appellation: engine <module>
    Created At: 2025.08.30:00:15:55
    Contrib: @FL03
*/
use super::Handle;

use crate::error::Error;
use rstm_core::state::RawState;
use rstm_rules::{Rule, Tail};

#[doc(hidden)]
pub trait Engine<Q, S>: Handle<Tail<Q, S>>
where
    Q: RawState,
{
    fn load<I>(&mut self, program: I)
    where
        I: IntoIterator<Item = Rule<Q, S>>;

    fn run(&mut self) -> Result<(), Error>;
}
