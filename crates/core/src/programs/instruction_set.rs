/*
    Appellation: program_base <module>
    Created At: 2026.01.11:12:34:11
    Contrib: @FL03
*/
use super::RawRuleset;
use crate::{Head, Rule, Tail};
use rstm_state::{RawState, State};

#[cfg(feature = "hashbrown")]
use hashbrown::{HashMap, HashSet};
#[cfg(all(not(feature = "hashbrown"), feature = "std"))]
use std::collections::{HashMap, HashSet};

pub type ProgramSlice<Q = isize, A = char> = ProgramBase<[Rule<Q, A>], Q, A>;
pub type ProgramArray<Q, A, const N: usize> = ProgramBase<[Rule<Q, A>; N], Q, A>;
#[cfg(feature = "alloc")]
/// a type alias for a [`ProgramBase`] using a [`Vec`](alloc::vec::Vec) as the ruleset
pub type Program<Q, A> = ProgramBase<alloc::vec::Vec<Rule<Q, A>>, Q, A>;
#[cfg(any(feature = "hashbrown", feature = "std"))]
/// a type alias for a [`ProgramBase`] using a [`HashMap`] as the ruleset, using the head
/// as key and the tail as value
pub type ProgramMap<Q, A> = ProgramBase<HashMap<Head<Q, A>, Tail<Q, A>>, Q, A>;
#[cfg(any(feature = "hashbrown", feature = "std"))]
/// a type alias for a [`ProgramBase`] using a [`HashSet`] consisting of rules as the
/// store
pub type ProgramSet<Q, A> = ProgramBase<HashSet<Rule<Q, A>>, Q, A>;

/// The [`ProgramBase`] implementation is a generic structure representing a set of
/// instructions or rules capable of transforming states within a computational model.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(deny_unknown_fields, rename_all = "snake_case")
)]
#[repr(C)]
pub struct ProgramBase<R: ?Sized, Q, A>
where
    Q: RawState,
    R: RawRuleset<Q, A>,
{
    pub(crate) initial_state: Option<State<Q>>,
    pub(crate) _marker: core::marker::PhantomData<Rule<Q, A>>,
    pub(crate) rules: R,
}
