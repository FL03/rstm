/*
    Appellation: rulespace <module>
    Created At: 2025.08.30:08:03:10
    Contrib: @FL03
*/
use rstm_core::{Head, Tail};
use rstm_state::RawState;

pub trait RawPoint {
    type Key;
    type Value;
}

/// The [`RawSpace`] trait defines the basic interface for any compatible stores used within
/// the crate.
pub trait RawSpace {
    /// The type used to index into the storage structure.
    type Key;
    /// The type of values stored within the structure.
    type Value;

    private! {}
}
/// The [`RuleSpace`] extends the [`RawSpace`] trait to introduce rule-specific functionality.
/// It provides a method to retrieve the tail of a rule given its head.
pub trait RuleSpace<Q, S>: RawSpace
where
    Q: RawState,
{
    /// returns the tail associated with the provided head, if it exists
    fn get(&self, head: &Head<Q, S>) -> Option<&Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq;
    /// returns a mutable reference to the tail associated with the provided head, if it exists
    fn get_mut(&mut self, head: &Head<Q, S>) -> Option<&mut Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq;
    /// inserts a new rule into the storage structure
    fn insert(&mut self, head: Head<Q, S>, tail: Tail<Q, S>);
}

/*
 ************* Implementations *************
*/

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{RawSpace, RuleSpace};
    use alloc::vec::Vec;
    use rstm_core::{Head, Rule, Tail};
    use rstm_state::RawState;

    impl<T> RawSpace for Vec<T> {
        type Key = usize;
        type Value = T;

        seal! {}
    }

    impl<Q, A> RuleSpace<Q, A> for Vec<Rule<Q, A>>
    where
        Q: RawState + PartialEq,
        A: PartialEq,
    {
        fn get(&self, key: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            self.iter().find_map(|rule| {
                if rule.head() == key {
                    Some(rule.tail())
                } else {
                    None
                }
            })
        }

        fn get_mut(&mut self, key: &Head<Q, A>) -> Option<&mut Tail<Q, A>> {
            self.iter_mut().find_map(|rule| {
                if rule.head() == key {
                    Some(rule.tail_mut())
                } else {
                    None
                }
            })
        }

        fn insert(&mut self, head: Head<Q, A>, tail: Tail<Q, A>) {
            self.push(Rule { head, tail });
        }
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::{RawSpace, RuleSpace};

    use rstm_core::{Head, Tail};
    use rstm_state::RawState;
    use std::collections::HashMap;

    impl<K, V> RawSpace for HashMap<K, V> {
        type Key = K;
        type Value = V;

        seal! {}
    }

    impl<Q, A> RuleSpace<Q, A> for HashMap<Head<Q, A>, Tail<Q, A>>
    where
        Q: RawState + Eq + core::hash::Hash,
        A: Eq + core::hash::Hash,
    {
        fn get(&self, key: &Head<Q, A>) -> Option<&Tail<Q, A>> {
            self.get(key)
        }

        fn get_mut(&mut self, key: &Head<Q, A>) -> Option<&mut Tail<Q, A>> {
            self.get_mut(key)
        }

        fn insert(&mut self, head: Head<Q, A>, tail: Tail<Q, A>) {
            self.insert(head, tail);
        }
    }
}
