/*
    Appellation: aliases <module>
    Created At: 2026.01.11:15:06:06
    Contrib: @FL03
*/
use crate::programs::program_base::ProgramBase;
use crate::rules::Rule;
#[cfg(feature = "alloc")]
use crate::rules::{Head, Tail};

#[cfg(feature = "alloc")]
use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};
#[cfg(feature = "hashbrown")]
use hashbrown::{HashMap, HashSet};
#[cfg(all(not(feature = "hashbrown"), feature = "std"))]
use std::collections::{HashMap, HashSet};

/// A type alias for a [`ProgramBase`] configured to use a slice of rules as the ruleset, with default generic parameters for the state and action types
pub type ProgramSlice<Q = isize, A = char> = ProgramBase<[Rule<Q, A>], Q, A>;

pub type ProgramArray<Q, A, const N: usize> = ProgramBase<[Rule<Q, A>; N], Q, A>;
#[cfg(feature = "alloc")]
/// a type alias for a [`ProgramBase`] using a [`Vec`] as the ruleset
pub type Program<Q, A> = ProgramBase<Vec<Rule<Q, A>>, Q, A>;

#[cfg(feature = "alloc")]
pub type ProgramBSet<Q, A> = ProgramBase<BTreeSet<Rule<Q, A>>, Q, A>;

#[cfg(feature = "alloc")]
pub type ProgramBMap<Q, A> = ProgramBase<BTreeMap<Head<Q, A>, Tail<Q, A>>, Q, A>;

#[cfg(any(feature = "hashbrown", feature = "std"))]
/// a type alias for a [`ProgramBase`] using a [`HashMap`] as the ruleset, using the head
/// as key and the tail as value
pub type ProgramMap<Q, A> = ProgramBase<HashMap<Head<Q, A>, Tail<Q, A>>, Q, A>;
#[cfg(any(feature = "hashbrown", feature = "std"))]
/// a type alias for a [`ProgramBase`] using a [`HashSet`] consisting of rules as the
/// store
pub type ProgramSet<Q, A> = ProgramBase<HashSet<Rule<Q, A>>, Q, A>;
