/*
    Appellation: program_base <module>
    Created At: 2026.01.11:12:34:11
    Contrib: @FL03
*/
use super::RuleSet;
use crate::{Head, Rule, Tail};
use rstm_state::State;

#[cfg(feature = "hashbrown")]
use hashbrown::{HashMap, HashSet};
#[cfg(all(not(feature = "hashbrown"), feature = "std"))]
use std::collections::{HashMap, HashSet};

pub type ProgramArray<Q, A, const N: usize> = InstructionSet<Q, A, [Rule<Q, A>; N]>;
pub type ProgramSlice<Q, A> = InstructionSet<Q, A, [Rule<Q, A>]>;
#[cfg(feature = "alloc")]
/// a type alias for a [`ProgramBase`] using a [`Vec`](alloc::vec::Vec) as the ruleset
pub type ProgramVec<Q, A> = InstructionSet<Q, A, alloc::vec::Vec<Rule<Q, A>>>;
#[cfg(any(feature = "hashbrown", feature = "std"))]
/// a type alias for a [`ProgramBase`] using a [`HashMap`] as the ruleset, using the head as key and the tail as value
pub type ProgramMap<Q, A> = InstructionSet<Q, A, HashMap<Head<Q, A>, Tail<Q, A>>>;
#[cfg(any(feature = "hashbrown", feature = "std"))]
/// a type alias for a [`ProgramBase`] using a [`HashSet`] as the ruleset, using the head as key and the tail as value
pub type ProgramSet<Q, A> = InstructionSet<Q, A, HashSet<Rule<Q, A>>>;

/// The [`ProgramBase`]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
pub struct InstructionSet<R, Q, A>
where
    R: RuleSet<Q, A>,
{
    pub(crate) rules: R,
    pub(crate) initial_state: Option<State<Q>>,
    pub(crate) _marker: core::marker::PhantomData<(Head<Q, A>, Tail<Q, A>)>,
}
