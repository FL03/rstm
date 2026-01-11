/*
    Appellation: impl_turing_engine <module>
    Created At: 2025.08.31:14:48:57
    Contrib: @FL03
*/
use crate::actors::engine::TuringEngine;
use rstm_state::RawState;

#[doc(hidden)]
impl<Q, A> TuringEngine<'_, Q, A>
where
    Q: RawState,
{
    #[deprecated(since = "0.0.9", note = "use `current_epoch` instead")]
    pub const fn steps(&self) -> usize {
        self.cycles
    }
}
