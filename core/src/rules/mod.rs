/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{
    rule::{Rule, RuleBuilder},
    rule_map::RuleMap,
    rule_set::RuleSet,
};

pub(crate) mod rule;

pub mod rule_map;
pub mod rule_set;

#[doc(hidden)]
mod impls {
    pub mod impl_rule;
    pub mod impl_rule_repr;
}

pub(crate) mod prelude {
    pub use super::rule::Rule;
    pub use super::rule_set::RuleSet;
    pub use super::{Directive, Scope, Transition};
}

use crate::{Direction, Head, State, Symbolic, Tail};

pub trait Program<Q, A> {
    type Key: Scope<Q, A>;
    type Val: Directive<Q, A>;

    fn contains_key(&self, key: &Self::Key) -> bool;

    fn get(&self, key: &Self::Key) -> Option<&Self::Val>;

    fn insert(&mut self, key: Self::Key, val: Self::Val) -> Option<Self::Val>;

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Val>;
}

pub trait Transition<Q, S> {
    fn direction(&self) -> Direction;

    fn current_state(&self) -> State<&'_ Q>;

    fn next_state(&self) -> State<&'_ Q>;

    fn symbol(&self) -> &S;

    fn write_symbol(&self) -> &S;

    fn head(&self) -> Head<&Q, &S> {
        Head {
            state: self.current_state(),
            symbol: self.symbol(),
        }
    }

    fn tail(&self) -> Tail<&Q, &S> {
        Tail {
            direction: self.direction(),
            state: self.next_state(),
            symbol: self.write_symbol(),
        }
    }

    fn as_rule(&self) -> Rule<&Q, &S> {
        Rule {
            head: self.head(),
            tail: self.tail(),
        }
    }
}

/// The [`Scope`] trait is used to describe objects containing information or references to the
/// current state and symbol of a Turing machine.
pub trait Scope<Q, S> {
    fn current_state(&self) -> State<&'_ Q>;

    fn current_symbol(&self) -> &S;
}

/// [`Directive`] is a trait describing the `tail` of a typical Turing machine;
pub trait Directive<Q, S> {
    fn direction(&self) -> Direction;

    fn next_state(&self) -> State<&'_ Q>;

    fn next_symbol(&self) -> &S;
}

/*
 ************* Implementations *************
*/
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::collections::HashMap;

#[cfg(feature = "alloc")]
impl<Q, A> Program<Q, A> for Vec<Rule<Q, A>>
where
    Q: PartialEq,
    A: PartialEq,
{
    type Key = Head<Q, A>;
    type Val = Tail<Q, A>;

    fn contains_key(&self, key: &Head<Q, A>) -> bool {
        self.iter().any(|rule| rule.head() == key)
    }

    fn get(&self, key: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        self.iter().find_map(|rule| {
            if rule.head() == key {
                Some(rule.tail())
            } else {
                None
            }
        })
    }

    fn insert(&mut self, key: Head<Q, A>, val: Tail<Q, A>) -> Option<Tail<Q, A>> {
        self.iter_mut()
            .find(|rule| rule.head() == &key)
            .map(|rule| core::mem::replace(rule.tail_mut(), val))
    }

    fn remove(&mut self, key: &Head<Q, A>) -> Option<Tail<Q, A>> {
        let index = self.iter().position(|rule| rule.head() == key)?;
        Some(self.remove(index).tail)
    }
}

#[cfg(feature = "std")]
impl<Q, A> Program<Q, A> for HashMap<Head<Q, A>, Tail<Q, A>>
where
    Q: Eq + core::hash::Hash,
    A: Eq + core::hash::Hash,
{
    type Key = Head<Q, A>;
    type Val = Tail<Q, A>;

    fn contains_key(&self, key: &Head<Q, A>) -> bool {
        self.contains_key(key)
    }

    fn get(&self, key: &Head<Q, A>) -> Option<&Tail<Q, A>> {
        self.get(key)
    }

    fn insert(&mut self, key: Head<Q, A>, val: Tail<Q, A>) -> Option<Tail<Q, A>> {
        self.insert(key, val)
    }

    fn remove(&mut self, key: &Head<Q, A>) -> Option<Tail<Q, A>> {
        self.remove(key)
    }
}

impl<A, Q, S> Transition<Q, S> for A
where
    A: Scope<Q, S> + Directive<Q, S>,
    S: Symbolic,
{
    fn direction(&self) -> Direction {
        self.direction()
    }

    fn current_state(&self) -> State<&'_ Q> {
        self.current_state()
    }

    fn next_state(&self) -> State<&'_ Q> {
        self.next_state()
    }

    fn symbol(&self) -> &S {
        self.current_symbol()
    }

    fn write_symbol(&self) -> &S {
        self.next_symbol()
    }
}

impl<Q, S> Scope<Q, S> for (State<Q>, S) {
    fn current_state(&self) -> State<&'_ Q> {
        self.0.to_ref()
    }

    fn current_symbol(&self) -> &S {
        &self.1
    }
}

impl<Q, S> Scope<Q, S> for crate::Head<Q, S> {
    fn current_state(&self) -> State<&'_ Q> {
        self.state()
    }

    fn current_symbol(&self) -> &S {
        &self.symbol
    }
}

impl<Q, S> Scope<Q, S> for Rule<Q, S> {
    fn current_state(&self) -> State<&'_ Q> {
        self.head.state.to_ref()
    }

    fn current_symbol(&self) -> &S {
        self.symbol()
    }
}

impl<Q, S> Directive<Q, S> for (Direction, State<Q>, S) {
    fn direction(&self) -> Direction {
        self.0
    }

    fn next_state(&self) -> State<&'_ Q> {
        self.1.to_ref()
    }

    fn next_symbol(&self) -> &S {
        &self.2
    }
}

impl<Q, S> Directive<Q, S> for crate::Tail<Q, S> {
    fn direction(&self) -> Direction {
        self.direction
    }

    fn next_state(&self) -> State<&'_ Q> {
        self.state()
    }

    fn next_symbol(&self) -> &S {
        self.symbol()
    }
}

impl<Q, S> Directive<Q, S> for Rule<Q, S> {
    fn direction(&self) -> Direction {
        self.direction()
    }

    fn next_state(&self) -> State<&'_ Q> {
        self.tail().state()
    }

    fn next_symbol(&self) -> &S {
        self.write_symbol()
    }
}
