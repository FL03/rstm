/*
    Appellation: impl_engine_repr <module>
    Created At: 2026.01.17:21:19:21
    Contrib: @FL03
*/
use crate::actors::EngineBase;
use crate::rules::Head;
use rstm_state::RawState;

impl<Q, A> EngineBase<Head<Q, usize>, Q, A>
where
    Q: RawState,
{
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
