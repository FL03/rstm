/*
    Appellation: engine <module>
    Created At: 2025.08.31:14:49:50
    Contrib: @FL03
*/
use super::Driver;
use crate::programs::Program;
use crate::rules::Head;
use alloc::vec::Vec;
use rstm_state::RawState;

#[deprecated(
    since = "0.1.3",
    note = "The `TMHEngine` alias has been deprecated; please use the `HeadEngine` instead."
)]
#[allow(deprecated)]
/// A type alias for a [`EngineBase`] configured with a _moving head_ execution model, or
/// driver.
pub type TMHEngine<Q, A> = EngineBase<super::TMH<Q, A>, Q, A>;
/// A type alias for an [`EngineBase`] instance configured with a _moving head_ model using
/// the [`Head<Q, usize>`] structure to maintain the head's position on the tape.
pub type HeadEngine<Q, A> = EngineBase<Head<Q, usize>, Q, A>;
/// The [`EngineBase`] implementation is designed as a type of runtime for executing various
/// Turing machine models, or drivers, according to a specified set of rules encapsulated
/// within a [`Program<Q, A>`].
pub struct EngineBase<D, Q, A>
where
    D: Driver<Q, A>,
    Q: RawState,
{
    /// the actor that will be executing the program
    pub(crate) driver: D,
    /// the program being executed
    pub(crate) program: Option<Program<Q, A>>,
    /// the number of cycles executed; independent of the position of the head on the tape
    pub(crate) cycles: usize,
    /// the output tape captures the results of the execution
    pub(crate) tape: Vec<A>,
}
