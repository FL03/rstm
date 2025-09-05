/*
    Appellation: impl_ruliad <module>
    Created At: 2025.09.04:19:08:22
    Contrib: @FL03
*/
use super::Ruliad;
use crate::types::RuleVec;
use rstm_core::{Head, Rule, Tail};
use rstm_state::{RawState, State};

use alloc::vec::Vec;

impl<Q, S> Ruliad<Q, S>
where
    Q: RawState,
{
    /// returns a new, empty instance of the [`Ruliad`]
    pub const fn new() -> Self {
        Self {
            rules: RuleVec::new(),
        }
    }
    /// returns a new instance of the ruliad from the given iterator
    pub fn from_rules<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Rule<Q, S>>,
    {
        Self {
            rules: RuleVec::from_iter(iter),
        }
    }
    /// Returns a reference to the instructions.
    pub const fn rules(&self) -> &RuleVec<Q, S> {
        &self.rules
    }
    /// Returns a mutable reference to the instructions.
    pub const fn rules_mut(&mut self) -> &mut RuleVec<Q, S> {
        &mut self.rules
    }
    /// consumes the current instance to create another with the given rules
    pub fn with_rules<I>(self, rules: I) -> Self
    where
        I: IntoIterator<Item = Rule<Q, S>>,
    {
        Self {
            rules: Vec::from_iter(rules),
            ..self
        }
    }
    /// Returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<'_, Rule<Q, S>> {
        self.rules().iter()
    }
    /// Returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, Rule<Q, S>> {
        self.rules_mut().iter_mut()
    }
    /// returns an immutable reference to the tail for a given head; returns [`None`](Option::None)
    /// if no match is found.
    pub fn get(&self, head: &Head<Q, S>) -> Option<&Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter().find_map(|i| {
            if i.head() == head {
                Some(i.tail())
            } else {
                None
            }
        })
    }
    /// returns a mutable reference to the tail for a given head; returns [`None`](Option::None)
    /// if no match is found.
    pub fn get_mut(&mut self, head: &Head<Q, S>) -> Option<&mut Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter_mut().find_map(|i| {
            if i.head() == head {
                Some(i.tail_mut())
            } else {
                None
            }
        })
    }
    /// returns an immutable reference to the tail of a rule whose state and symbol match the
    /// givens
    pub fn find_tail(&self, state: State<&Q>, symbol: &S) -> Option<&Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter().find_map(|i| {
            if i.head().view() == (Head { state, symbol }) {
                Some(i.tail())
            } else {
                None
            }
        })
    }
    /// returns a mutable reference to the tail for a given head; returns [`None`](Option::None)
    /// if no match is found.
    pub fn find_mut_tail(&mut self, head: Head<&Q, &S>) -> Option<&mut Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter_mut().find_map(|i| {
            if i.head().view() == head {
                Some(i.tail_mut())
            } else {
                None
            }
        })
    }
    /// Returns a collection of rules whose head contains a match for the given state.
    pub fn filter_by_state(&self, state: State<&Q>) -> Vec<&Rule<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter().filter(|i| *i.head() == state).collect()
    }
}
