/*
    appellation: impl_rule_set <module>
    authors: @FL03
*/
use crate::ruleset::Program;
use crate::{Rule, RuleVec};
use rstm_core::{Head, Tail};
use rstm_state::{RawState, State};

use alloc::vec::{self, Vec};

impl<Q, S> Program<Q, S>
where
    Q: RawState,
{
    pub fn new() -> Self {
        Self {
            initial_state: None,
            rules: RuleVec::new(),
        }
    }
    #[allow(clippy::should_implement_trait)]
    /// Create a new instance of the [Program] from the given rules.
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Rule<Q, S>>,
    {
        Self {
            initial_state: None,
            rules: RuleVec::from_iter(iter),
        }
    }
    /// Create a new instance of the [Program] using the given initial state.
    pub fn from_state(initial_state: State<Q>) -> Self {
        Self {
            initial_state: Some(initial_state),
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

    /// Configures the program to use the given initial state.
    pub fn with_initial_state(self, state: State<Q>) -> Self {
        Self {
            initial_state: Some(state),
            ..self
        }
    }
    /// Configures the program with the given rules;
    pub fn with_rules<I>(self, instructions: I) -> Self
    where
        I: IntoIterator<Item = Rule<Q, S>>,
    {
        Self {
            rules: Vec::from_iter(instructions),
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
    /// Returns a collection of tails for a given head.
    pub fn get_tail_with(&self, State(state): State<&Q>, symbol: &S) -> Option<&Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter().find_map(|i| {
            if i.head().state() == state && i.head().symbol() == symbol {
                Some(i.tail())
            } else {
                None
            }
        })
    }
    /// Returns a mutable reference to a the tail matching the given head.
    pub fn get_mut_tail_with(
        &mut self,
        State(state): State<&Q>,
        symbol: &S,
    ) -> Option<&mut Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter_mut().find_map(|i| {
            if i.head().state() == state && i.head().symbol() == symbol {
                Some(i.tail_mut())
            } else {
                None
            }
        })
    }
    /// Returns a collection of tails for a given head.
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
    /// Returns a collection of tails for a given head.
    pub fn get_ref(&self, head: Head<&Q, &S>) -> Option<Tail<&Q, &S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter().find_map(|i| {
            if i.head_view() == head {
                Some(i.tail_view())
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
    fn from(instructions: Vec<Rule<Q, S>>) -> Self {
        Self {
            initial_state: Some(State::default()),
            rules: instructions,
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
        Self {
            initial_state: Some(State::default()),
            rules: RuleVec::from_iter(iter),
        }
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
