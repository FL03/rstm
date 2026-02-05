/*
    Appellation: program_base <module>
    Created At: 2026.01.11:12:34:11
    Contrib: @FL03
*/
use super::RawRuleset;
use crate::rules::Instruction;
use rstm_state::{RawState, State};

/// The [`ProgramBase`] struct is used to define a generic program capable of being executed by
/// a Turing machine or similar computational model. It consists of an optional initial state,
/// a set of rules (or instructions) used to indicate how the machine should *respond* under
/// different *circumstances*, and a marker to associate the generic parameters with the struct.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
#[repr(C)]
pub struct ProgramBase<R: ?Sized, Q, A, I = <R as RawRuleset<Q, A>>::Rule>
where
    Q: RawState,
    R: RawRuleset<Q, A, Rule = I>,
    I: Instruction<Q, A>,
{
    pub(crate) initial_state: Option<State<Q>>,
    pub(crate) _marker: core::marker::PhantomData<(I, Q, A)>,
    pub(crate) rules: R,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{Direction, Rule};

    #[test]
    fn test_program_base() {
        let rule = Rule::from_parts(0, 'a', Direction::Right, 1, 'b');

        let program = ProgramBase {
            initial_state: Some(State::new(0)),
            _marker: core::marker::PhantomData,
            rules: [rule],
        };
        assert_eq!(program.initial_state, Some(State::new(0)));
    }
}
