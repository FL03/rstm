/*
    Appellation: impl_engine_repr <module>
    Created At: 2026.01.17:21:19:21
    Contrib: @FL03
*/
use crate::actors::EngineBase;
use crate::programs::Program;
use crate::rules::Head;
use rstm_state::RawState;

impl<Q, A> EngineBase<Head<Q, usize>, Q, A>
where
    Q: RawState + PartialEq,
    A: PartialEq,
{
    /// initialize a new instance of a turing machine with a moving head using the given
    /// program.
    pub fn tmh(program: Program<Q, A>) -> Self
    where
        Q: Clone + Default,
    {
        Self {
            driver: Head {
                state: program.initial_state().cloned().unwrap_or_default(),
                symbol: 0,
            },
            tape: Vec::new(),
            program: Some(program),
            cycles: 0,
        }
    }
    /// initialize a new instance of the TMH engine from the given state and input
    pub fn from_state_with_input<I>(state: Q, input: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        let driver = Head::new(state, 0);
        Self {
            driver,
            tape: Vec::from_iter(input),
            program: None,
            cycles: 0,
        }
    }
}
