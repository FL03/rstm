/*
    Appellation: aliases <module>
    Created At: 2026.01.11:13:26:21
    Contrib: @FL03
*/
#[cfg(any(feature = "hashbrown", feature = "std"))]
pub use self::hash_based::*;
#[cfg(feature = "alloc")]
pub use self::impl_alloc::*;

use crate::{Head, Rule, Tail};

/// a type alias for a slice whose elements are 2-tuples consisting of a [`Head`] and a [`Tail`]
pub type TupleRuleSlice<Q, A> = [(Head<Q, A>, Tail<Q, A>)];
/// a type alias for an array whose elements are 2-tuples consisting of a [`Head`] and a [`Tail`]
pub type TupleRuleArray<Q, A, const N: usize> = [(Head<Q, A>, Tail<Q, A>); N];
/// A type alias for a slice containing [`Rule`] elements.
pub type RuleSlice<Q, A> = [Rule<Q, A>];
/// a type alias for an array of [`Rule`]'s
pub type RuleArray<Q, A, const N: usize> = [Rule<Q, A>; N];

#[cfg(feature = "alloc")]
mod impl_alloc {
    use crate::{Head, Rule, Tail};
    use alloc::vec::Vec;
    /// A type alias for a [`Vec`] containing [`Rule`] elements.
    pub type RuleVec<Q, S> = Vec<Rule<Q, S>>;

    pub type HeadVec<Q = usize, A = usize> = Vec<Head<Q, A>>;

    pub type TailVec<Q = usize, A = usize> = Vec<Tail<Q, A>>;
}

#[cfg(any(feature = "hashbrown", feature = "std"))]
mod hash_based {
    use crate::{Head, Rule, Tail};
    #[cfg(feature = "hashbrown")]
    use hashbrown::{HashMap, HashSet};
    #[cfg(all(not(feature = "hashbrown"), feature = "std"))]
    use std::collections::{HashMap, HashSet};

    /// A type alias for a [`HashMap`] where each key is a [`Head`] and each value is a [`Tail`].
    pub type HeadMap<Q = usize, A = usize> = HashMap<Head<Q, A>, Tail<Q, A>>;
    /// A type alias for a [`HashMap`] with `usize` keys and [`Rule`] values.
    pub type RuleHashMap<Q, A> = HashMap<usize, Rule<Q, A>>;
    /// A type alias for a [`HashSet`] containing [`Rule`] elements.
    pub type RuleHashSet<Q, A> = HashSet<Rule<Q, A>>;
}
