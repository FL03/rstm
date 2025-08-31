/*
    appellation: impl_rule_set <module>
    authors: @FL03
*/
use crate::program::Program;
use crate::{Rule, RuleVec};
use rstm_core::{Head, Tail};
use rstm_state::{RawState, State};

use alloc::vec::{self, Vec};

impl<Q, S> Program<Q, S>
where
    Q: RawState,
{
    /// returns a new, empty instance of the [`Program`]
    pub const fn new() -> Self {
        Self {
            initial_state: None,
            rules: RuleVec::new(),
        }
    }
    /// returns a new instance of the [`Program`] using the given rules 
    pub fn from_rules<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Rule<Q, S>>,
    {
        Self {
            initial_state: None,
            rules: RuleVec::from_iter(iter),
        }
    }
    /// Create a new instance of the [Program] using the given initial state.
    pub fn from_state(initial_state: Q) -> Self {
        Self {
            initial_state: Some(State(initial_state)),
            rules: RuleVec::new(),
        }
    }
    /// Returns an owned reference to the initial state of the program.
    pub fn initial_state(&self) -> Option<State<&'_ Q>> {
        self.initial_state.as_ref().map(|state| state.view())
    }
    /// Returns a reference to the instructions.
    pub const fn rules(&self) -> &RuleVec<Q, S> {
        &self.rules
    }
    /// Returns a mutable reference to the instructions.
    pub const fn rules_mut(&mut self) -> &mut RuleVec<Q, S> {
        &mut self.rules
    }
    /// consumes the current instance to create another with the given default state
    pub fn with_default_state(self, state: Q) -> Self {
        Self {
            initial_state: Some(State(state)),
            ..self
        }
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

impl<Q, S> AsRef<[Rule<Q, S>]> for Program<Q, S>
where
    Q: RawState,
{
    fn as_ref(&self) -> &[Rule<Q, S>] {
        self.rules()
    }
}

impl<Q, S> AsMut<[Rule<Q, S>]> for Program<Q, S>
where
    Q: RawState,
{
    fn as_mut(&mut self) -> &mut [Rule<Q, S>] {
        self.rules_mut()
    }
}

impl<Q, S> core::ops::Deref for Program<Q, S>
where
    Q: RawState,
{
    type Target = [Rule<Q, S>];

    fn deref(&self) -> &Self::Target {
        &self.rules
    }
}

impl<Q, S> core::ops::DerefMut for Program<Q, S>
where
    Q: RawState,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rules
    }
}

impl<Q, S> core::ops::Index<Head<Q, S>> for Program<Q, S>
where
    Q: RawState + PartialEq,
    S: PartialEq,
{
    type Output = Tail<Q, S>;

    fn index(&self, index: Head<Q, S>) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl<Q, S> From<Vec<Rule<Q, S>>> for Program<Q, S>
where
    Q: RawState + Default,
{
    fn from(rules: Vec<Rule<Q, S>>) -> Self {
        Self {
            initial_state: Some(State::default()),
            rules,
        }
    }
}

impl<Q, S> Extend<Rule<Q, S>> for Program<Q, S>
where
    Q: RawState,
{
    fn extend<I: IntoIterator<Item = Rule<Q, S>>>(&mut self, iter: I) {
        self.rules.extend(iter)
    }
}

impl<Q, S> FromIterator<Rule<Q, S>> for Program<Q, S>
where
    Q: RawState + Default,
{
    fn from_iter<I: IntoIterator<Item = Rule<Q, S>>>(iter: I) -> Self {
        let rules = RuleVec::from_iter(iter);
        Self::from_rules(rules)
    }
}

impl<Q, S> IntoIterator for Program<Q, S>
where
    Q: RawState,
{
    type Item = Rule<Q, S>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}
