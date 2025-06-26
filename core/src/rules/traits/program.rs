/*
    appellation: program <module>
    authors: @FL03
*/
use crate::Direction;
use crate::rules::{Head, Rule, Tail};
use crate::state::{RawState, State};

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

impl<Q, S> Scope<Q, S> for (State<Q>, S) {
    fn current_state(&self) -> &State<Q> {
        &self.0
    }

    fn current_symbol(&self) -> &S {
        &self.1
    }
}

impl<Q, S> Scope<Q, S> for Head<Q, S>
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

impl<Q, S> Directive<Q, S> for Tail<Q, S>
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
