/*
    appellation: impl_rule_set <module>
    authors: @FL03
*/
use crate::rules::{Rule, RuleSet, Rules};
use crate::state::{RawState, State};
use crate::{Head, Tail};

use alloc::vec::{self, Vec};

impl<Q, S> RuleSet<Q, S> where Q: RawState {}

impl<Q, S> AsRef<[Rule<Q, S>]> for RuleSet<Q, S>
where
    Q: RawState,
{
    fn as_ref(&self) -> &[Rule<Q, S>] {
        self.rules()
    }
}

impl<Q, S> AsMut<[Rule<Q, S>]> for RuleSet<Q, S>
where
    Q: RawState,
{
    fn as_mut(&mut self) -> &mut [Rule<Q, S>] {
        self.rules_mut()
    }
}

impl<Q, S> core::ops::Deref for RuleSet<Q, S>
where
    Q: RawState,
{
    type Target = [Rule<Q, S>];

    fn deref(&self) -> &Self::Target {
        &self.rules
    }
}

impl<Q, S> core::ops::DerefMut for RuleSet<Q, S>
where
    Q: RawState,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rules
    }
}

impl<Q, S> core::ops::Index<Head<Q, S>> for RuleSet<Q, S>
where
    Q: RawState + PartialEq,
    S: PartialEq,
{
    type Output = Tail<Q, S>;

    fn index(&self, index: Head<Q, S>) -> &Self::Output {
        self.get(&index).unwrap()
    }
}

impl<Q, S> From<Vec<Rule<Q, S>>> for RuleSet<Q, S>
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

impl<Q, S> Extend<Rule<Q, S>> for RuleSet<Q, S>
where
    Q: RawState,
{
    fn extend<I: IntoIterator<Item = Rule<Q, S>>>(&mut self, iter: I) {
        self.rules.extend(iter)
    }
}

impl<Q, S> FromIterator<Rule<Q, S>> for RuleSet<Q, S>
where
    Q: RawState + Default,
{
    fn from_iter<I: IntoIterator<Item = Rule<Q, S>>>(iter: I) -> Self {
        Self {
            initial_state: Some(State::default()),
            rules: Rules::from_iter(iter),
        }
    }
}

impl<Q, S> IntoIterator for RuleSet<Q, S>
where
    Q: RawState,
{
    type Item = Rule<Q, S>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}
