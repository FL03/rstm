/*
    Appellation: engine <module>
    Created At: 2025.08.31:14:49:50
    Contrib: @FL03
*/

mod impl_turing_engine;

use crate::tmh::TMH;
use rstm_rules::Program;
use rstm_state::RawState;

/// The [`TuringEngine`] implementation is designed to handle the execution of a given program.
/// The exact nature of the engine is determined, in part, by the type of _driver_ it employs
///
pub struct TuringEngine<'a, Q, A>
where
    Q: RawState,
{
    /// the actor that will be executing the program
    pub(crate) driver: &'a mut TMH<Q, A>,
    /// the program being executed
    pub(crate) program: Option<Program<Q, A>>,
    /// the number of steps taken by the actor
    pub(crate) steps: usize,
    pub(crate) _inputs: Vec<A>,
}
