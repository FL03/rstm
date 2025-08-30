/*
    appellation: impl_deprecated <module>
    authors: @FL03
*/
use crate::ruleset::InstructionSet;
use crate::types::{Head, RuleVec, Tail};
use rstm_state::RawState;

#[doc(hidden)]
impl<Q, S> InstructionSet<Q, S>
where
    Q: RawState + Default,
{
    #[deprecated(since = "0.0.8", note = "use `rules` instead")]
    pub const fn instructions(&self) -> &RuleVec<Q, S> {
        self.rules()
    }
    #[deprecated(since = "0.0.8", note = "use `rules_mut` instead")]
    pub fn instructions_mut(&mut self) -> &mut RuleVec<Q, S> {
        self.rules_mut()
    }
    #[deprecated(since = "0.0.8", note = "use `get` instead")]
    pub fn get_by_head(&self, head: &Head<Q, S>) -> Option<&Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.get(head)
    }
}
