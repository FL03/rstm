/*
    Appellation: engine <module>
    Created At: 2026.01.11:13:48:50
    Contrib: @FL03
*/

use super::Driver;

use crate::programs::Program;
use rstm_state::RawState;

/// The [`Executor`] trait defines the basis for compatible engines within the system.
pub trait Executor<Q, A>
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    type Driver: Driver<Q, A>;

    private!();

    fn load(&mut self, program: Program<Q, A>);

    fn run(&mut self) -> crate::Result<()>;
}
