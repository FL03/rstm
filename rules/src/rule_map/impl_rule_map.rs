/*
    appellation: impl_rule_map <module>
    authors: @FL03
*/
use super::RuleMap;
use crate::Rule;
use core::hash::Hash;
use rstm_core::{Head, Tail};
use rstm_state::RawState;
use std::collections::hash_map;

impl<Q, S> RuleMap<Q, S>
where
    Q: RawState + Eq + Hash,
    S: Eq + Hash,
{
}

impl<Q, S> core::ops::Index<Head<Q, S>> for RuleMap<Q, S>
where
    Q: RawState + Eq + Hash,
    S: Eq + Hash,
{
    type Output = Tail<Q, S>;

    fn index(&self, head: Head<Q, S>) -> &Self::Output {
        self.get(&head).expect("Rule not found")
    }
}
impl<Q, S> Extend<(Head<Q, S>, Tail<Q, S>)> for RuleMap<Q, S>
where
    Q: RawState + Eq + Hash,
    S: Eq + Hash,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (Head<Q, S>, Tail<Q, S>)>,
    {
        for (head, tail) in iter {
            self.insert(head, tail);
        }
    }
}

impl<Q, S> Extend<Rule<Q, S>> for RuleMap<Q, S>
where
    Q: RawState + Eq + Hash,
    S: Eq + Hash,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = Rule<Q, S>>,
    {
        for rule in iter {
            self.insert(rule.head, rule.tail);
        }
    }
}

impl<Q, S> FromIterator<(Head<Q, S>, Tail<Q, S>)> for RuleMap<Q, S>
where
    Q: RawState + Default + Eq + Hash,
    S: Eq + Hash,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Head<Q, S>, Tail<Q, S>)>,
    {
        Self::from_iter(iter)
    }
}

impl<Q, S> FromIterator<Rule<Q, S>> for RuleMap<Q, S>
where
    Q: RawState + Default + Eq + Hash,
    S: Eq + Hash,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Rule<Q, S>>,
    {
        Self::from_rules(iter)
    }
}

impl<Q, S> IntoIterator for RuleMap<Q, S>
where
    Q: RawState + Eq + Hash,
    S: Eq + Hash,
{
    type Item = (Head<Q, S>, Tail<Q, S>);
    type IntoIter = hash_map::IntoIter<Head<Q, S>, Tail<Q, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}

impl<'a, Q, S> IntoIterator for &'a RuleMap<Q, S>
where
    Q: RawState + Eq + Hash,
    S: Eq + Hash,
{
    type Item = (&'a Head<Q, S>, &'a Tail<Q, S>);
    type IntoIter = hash_map::Iter<'a, Head<Q, S>, Tail<Q, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.iter()
    }
}

impl<'a, Q, S> IntoIterator for &'a mut RuleMap<Q, S>
where
    Q: RawState + Eq + Hash,
    S: Eq + Hash,
{
    type Item = (&'a Head<Q, S>, &'a mut Tail<Q, S>);
    type IntoIter = hash_map::IterMut<'a, Head<Q, S>, Tail<Q, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.iter_mut()
    }
}
