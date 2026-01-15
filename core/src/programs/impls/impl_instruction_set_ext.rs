/*
    Appellation: impl_program <module>
    Created At: 2026.01.11:12:33:32
    Contrib: @FL03
*/
use crate::programs::{Instruction, InstructionSet, RuleSet};
use rstm_state::RawState;

impl<R, Q, A> core::fmt::Display for InstructionSet<R, Q, A>
where
    R: RuleSet<Q, A> + core::fmt::Debug,
    Q: RawState + core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(initial_state) = &self.initial_state {
            write! {
                f,
                "{{ initial_state: {}, rules: {:?} }}",
                initial_state, self.rules
            }
        } else {
            write! { f, "{{ rules: {:?} }}", self.rules }
        }
    }
}

impl<X, R, Q, A> FromIterator<X> for InstructionSet<R, Q, A>
where
    Q: RawState,
    X: Instruction<Q, A>,
    R: RuleSet<Q, A> + FromIterator<X>,
{
    fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: IntoIterator<Item = X>,
    {
        Self::from_rules(<R>::from_iter(iter))
    }
}
