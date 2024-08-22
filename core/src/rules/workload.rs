/*
    Appellation: workload <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
// #![cfg(feature = "std")]
use super::Rule;
use crate::{Head, State, Tail};
use std::collections::HashMap;

pub struct RuleSet<Q, S> {
    pub(crate) initial_state: State<Q>,
    pub(crate) rules: HashMap<Head<Q, S>, Tail<Q, S>>,
}

impl<Q, S> RuleSet<Q, S> {
    pub fn new() -> Self
    where
        Q: Default,
    {
        Self {
            initial_state: State::default(),
            rules: HashMap::new(),
        }
    }

    pub fn from_state(State(initial_state): State<Q>) -> Self {
        Self {
            initial_state: State(initial_state),
            rules: HashMap::new(),
        }
    }

    pub fn with_initial_state(self, State(state): State<Q>) -> Self {
        Self {
            initial_state: State(state),
            ..self
        }
    }
    /// Returns an instance of [State] which owns a reference to the interval value.
    pub fn initial_state(&self) -> State<&'_ Q> {
        self.initial_state.to_ref()
    }
    /// Returns an immutable reference to the set of rules.
    pub const fn rules(&self) -> &HashMap<Head<Q, S>, Tail<Q, S>> {
        &self.rules
    }
    /// Returns a mutable reference to the set of rules.
    pub fn rules_mut(&mut self) -> &mut HashMap<Head<Q, S>, Tail<Q, S>> {
        &mut self.rules
    }
}

impl<Q, S> RuleSet<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: Eq + core::hash::Hash,
{
    pub fn from_iter<I>(iter: I) -> Self
    where
        Q: Default,
        I: IntoIterator<Item = (Head<Q, S>, Tail<Q, S>)>,
    {
        Self {
            initial_state: State::default(),
            rules: HashMap::from_iter(iter),
        }
    }

    pub fn from_rules<I>(iter: I) -> Self
    where
        Q: Default,
        I: IntoIterator<Item = super::Rule<Q, S>>,
    {
        let mut rules = HashMap::new();
        for rule in iter {
            rules.insert(rule.head, rule.tail);
        }
        Self {
            initial_state: State::default(),
            rules,
        }
    }

    pub fn with_instructions(
        self,
        instructions: impl IntoIterator<Item = (Head<Q, S>, Tail<Q, S>)>,
    ) -> Self {
        let mut rules = HashMap::new();
        for (head, tail) in instructions {
            rules.insert(head, tail);
        }
        Self { rules, ..self }
    }
    /// Clears the set of rules.
    pub fn clear(&mut self) {
        self.rules.clear();
    }
    /// Returns an immutable reference to the tail of the rule for the given head; returns none
    /// if the head is not found.
    pub fn get(&self, head: &Head<Q, S>) -> Option<&Tail<Q, S>> {
        self.rules.get(head)
    }
    /// Returns a mutable reference to the tail of the rule for the given head; returns none if
    /// the head is not found.
    pub fn get_mut(&mut self, head: &Head<Q, S>) -> Option<&mut Tail<Q, S>> {
        self.rules.get_mut(head)
    }
    /// Returns the tail of the rule for the given head; returns none if the head is not found
    /// within the set of rules.
    pub fn get_ref(&self, head: &Head<Q, S>) -> Option<Tail<&Q, &S>> {
        self.get(head).map(|tail| tail.to_ref())
    }
    /// Inserts a new rule into the set of rules.
    pub fn insert(&mut self, head: Head<Q, S>, tail: Tail<Q, S>) {
        self.rules.insert(head, tail);
    }
    /// Inserts a new rule into the set of rules.
    pub fn insert_rule(&mut self, rule: Rule<Q, S>) {
        self.insert(rule.head, rule.tail);
    }
    /// Returns true if the set of rules is empty.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Returns the number of rules in the set.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Removes a rule from the set of rules.
    pub fn remove(&mut self, head: &Head<Q, S>) -> Option<Tail<Q, S>> {
        self.rules.remove(head)
    }
}

impl<Q, S> core::iter::Extend<(Head<Q, S>, Tail<Q, S>)> for RuleSet<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: Eq + core::hash::Hash,
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

impl<Q, S> core::iter::Extend<Rule<Q, S>> for RuleSet<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: Eq + core::hash::Hash,
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

impl<Q, S> core::iter::FromIterator<(Head<Q, S>, Tail<Q, S>)> for RuleSet<Q, S>
where
    Q: Default + Eq + core::hash::Hash,
    S: Eq + core::hash::Hash,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Head<Q, S>, Tail<Q, S>)>,
    {
        Self::from_iter(iter)
    }
}

impl<Q, S> core::iter::FromIterator<Rule<Q, S>> for RuleSet<Q, S>
where
    Q: Default + Eq + core::hash::Hash,
    S: Eq + core::hash::Hash,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Rule<Q, S>>,
    {
        Self::from_rules(iter)
    }
}

impl<Q, S> core::iter::IntoIterator for RuleSet<Q, S> {
    type Item = (Head<Q, S>, Tail<Q, S>);
    type IntoIter = std::collections::hash_map::IntoIter<Head<Q, S>, Tail<Q, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}

impl<'a, Q, S> core::iter::IntoIterator for &'a RuleSet<Q, S> {
    type Item = (&'a Head<Q, S>, &'a Tail<Q, S>);
    type IntoIter = std::collections::hash_map::Iter<'a, Head<Q, S>, Tail<Q, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.iter()
    }
}

impl<'a, Q, S> core::iter::IntoIterator for &'a mut RuleSet<Q, S> {
    type Item = (&'a Head<Q, S>, &'a mut Tail<Q, S>);
    type IntoIter = std::collections::hash_map::IterMut<'a, Head<Q, S>, Tail<Q, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.iter_mut()
    }
}
