/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::rule::{Rule, RuleBuilder};

#[cfg(feature = "std")]
#[doc(inline)]
pub use self::rule_map::RuleMap;
#[cfg(feature = "alloc")]
#[doc(inline)]
pub use self::rule_set::*;

pub(crate) mod rule;

#[cfg(feature = "std")]
pub mod rule_map;
#[cfg(feature = "alloc")]
pub mod rule_set;

#[doc(hidden)]
mod impls {
    pub mod impl_rule;
    #[cfg(feature = "std")]
    pub mod impl_rule_map;
    #[cfg(feature = "alloc")]
    pub mod impl_rule_set;
}

pub(crate) mod prelude {
    pub use super::rule::{Rule, RuleBuilder};

    #[cfg(feature = "std")]
    #[doc(inline)]
    pub use super::rule_map::RuleMap;
    #[cfg(feature = "alloc")]
    #[doc(inline)]
    pub use super::rule_set::RuleSet;
    #[doc(inline)]
    pub use super::{Directive, Scope, Transition};
}

use crate::state::{RawState, State};
use crate::{Direction, Head, Symbolic, Tail};

/// The [`Program`] trait establishes a common interface for objects that represent a
/// collection of rules.
pub trait Program<Q, A>
where
    Q: RawState,
{
    type Key: Scope<Q, A>;
    type Val: Directive<Q, A>;

    fn contains_key(&self, key: &Self::Key) -> bool;

    fn get(&self, key: &Self::Key) -> Option<&Self::Val>;

    fn insert(&mut self, key: Self::Key, val: Self::Val) -> Option<Self::Val>;

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Val>;
}
/// The [`Transition`] trait defines the expected behaviors of a particular rule within a
/// Turing machine program.
pub trait Transition<Q, S>
where
    Q: RawState,
{
    /// returns a copy of the direction of the head
    fn direction(&self) -> Direction;
    /// returns a reference to the current state of the Turing machine
    fn current_state(&self) -> &State<Q>;
    /// returns a reference to the next state of the Turing machine
    fn next_state(&self) -> &State<Q>;
    /// returns a reference to the current symbol under the head
    fn symbol(&self) -> &S;
    /// returns a reference to the symbol to be written by the head
    fn write_symbol(&self) -> &S;
    /// returns an instance of [`Head`] containing references to the current state and symbol
    fn head(&self) -> Head<&Q, &S> {
        Head {
            state: self.current_state().view(),
            symbol: self.symbol(),
        }
    }
    /// returns an instance of [`Tail`] containing references to the next state and symbol
    fn tail(&self) -> Tail<&Q, &S> {
        Tail {
            direction: self.direction(),
            state: self.next_state().view(),
            symbol: self.write_symbol(),
        }
    }
    /// returns an instance of [`Rule`] containing references to the states and symbols within
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
    fn current_state(&self) -> &State<Q>;

    fn current_symbol(&self) -> &S;
}

/// [`Directive`] is a trait describing the `tail` of a typical Turing machine;
pub trait Directive<Q, S> {
    fn direction(&self) -> Direction;

    fn next_state(&self) -> &State<Q>;

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
    Q: RawState + PartialEq,
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
    Q: RawState + Eq + core::hash::Hash,
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
    Q: RawState,
    S: Symbolic,
{
    fn direction(&self) -> Direction {
        self.direction()
    }

    fn current_state(&self) -> &State<Q> {
        self.current_state()
    }

    fn next_state(&self) -> &State<Q> {
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
    fn current_state(&self) -> &State<Q> {
        &self.0
    }

    fn current_symbol(&self) -> &S {
        &self.1
    }
}

impl<Q, S> Scope<Q, S> for crate::Head<Q, S>
where
    Q: RawState,
{
    fn current_state(&self) -> &State<Q> {
        self.state()
    }

    fn current_symbol(&self) -> &S {
        &self.symbol
    }
}

impl<Q, S> Scope<Q, S> for Rule<Q, S>
where
    Q: RawState,
{
    fn current_state(&self) -> &State<Q> {
        self.state()
    }

    fn current_symbol(&self) -> &S {
        self.symbol()
    }
}

impl<Q, S> Directive<Q, S> for (Direction, State<Q>, S)
where
    Q: RawState,
{
    fn direction(&self) -> Direction {
        self.0
    }

    fn next_state(&self) -> &State<Q> {
        &self.1
    }

    fn next_symbol(&self) -> &S {
        &self.2
    }
}

impl<Q, S> Directive<Q, S> for crate::Tail<Q, S>
where
    Q: RawState,
{
    fn direction(&self) -> Direction {
        self.direction
    }

    fn next_state(&self) -> &State<Q> {
        self.state()
    }

    fn next_symbol(&self) -> &S {
        self.symbol()
    }
}

impl<Q, S> Directive<Q, S> for Rule<Q, S>
where
    Q: RawState,
{
    fn direction(&self) -> Direction {
        self.direction()
    }

    fn next_state(&self) -> &State<Q> {
        self.tail().state()
    }

    fn next_symbol(&self) -> &S {
        self.write_symbol()
    }
}
