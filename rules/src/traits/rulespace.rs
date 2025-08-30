/*
    Appellation: rulespace <module>
    Created At: 2025.08.30:08:03:10
    Contrib: @FL03
*/
use crate::types::{Head, Tail};
use rstm_state::RawState;

pub trait RawSpace {
    private! {}
}

pub trait RuleSpace<Q, S>: RawSpace
where
    Q: RawState,
{
    fn get(&self, head: &Head<Q, S>) -> Option<&Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq;
}

/*
 ************* Implementations *************
*/

#[cfg(feature = "alloc")]
mod impl_alloc {
    use super::{RawSpace, RuleSpace};
    use crate::{Head, Rule, Tail};
    use alloc::vec::Vec;
    use rstm_state::RawState;

    impl<T> RawSpace for Vec<T> {
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
    }
}

#[cfg(feature = "std")]
mod impl_std {
    use super::{RawSpace, RuleSpace};

    use crate::{Head, Tail};
    use rstm_state::RawState;
    use std::collections::HashMap;

    impl<K, V> RawSpace for HashMap<K, V> {
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
    }
}
