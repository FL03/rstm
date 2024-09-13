/*
    Appellation: workload <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(feature = "std")]
use super::Rule;
use crate::{Head, State, Tail};
use std::collections::hash_map::{self, HashMap};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RuleMap<Q, S> {
    pub(crate) initial_state: Option<State<Q>>,
    pub(crate) rules: HashMap<Head<Q, S>, Tail<Q, S>>,
}

impl<Q, S> RuleMap<Q, S> {
    pub fn new() -> Self {
        Self {
            initial_state: None,
            rules: HashMap::new(),
        }
    }

    pub fn from_state(initial_state: State<Q>) -> Self {
        Self {
            initial_state: Some(initial_state),
            rules: HashMap::new(),
        }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
        I: IntoIterator<Item = (Head<Q, S>, Tail<Q, S>)>,
    {
        Self {
            initial_state: None,
            rules: HashMap::from_iter(iter),
        }
    }

    pub fn from_rules<I>(iter: I) -> Self
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
        I: IntoIterator<Item = super::Rule<Q, S>>,
    {
        Self::from_iter(iter.into_iter().map(|rule| (rule.head, rule.tail)))
    }
    /// configures the ruleset with the given initial state
    pub fn with_initial_state(self, state: State<Q>) -> Self {
        Self {
            initial_state: Some(state),
            ..self
        }
    }

    pub fn with_instructions(
        self,
        instructions: impl IntoIterator<Item = (Head<Q, S>, Tail<Q, S>)>,
    ) -> Self
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
    {
        Self {
            rules: HashMap::from_iter(instructions),
            ..self
        }
    }

    /// Returns an instance of [State] which owns a reference to the interval value.
    pub fn initial_state(&self) -> Option<State<&'_ Q>> {
        self.initial_state.as_ref().map(|state| state.to_ref())
    }

    /// Returns an immutable reference to the set of rules.
    pub const fn rules(&self) -> &HashMap<Head<Q, S>, Tail<Q, S>> {
        &self.rules
    }

    /// Returns a mutable reference to the set of rules.
    pub fn rules_mut(&mut self) -> &mut HashMap<Head<Q, S>, Tail<Q, S>> {
        &mut self.rules
    }

    /// Clears the set of rules.
    pub fn clear(&mut self) {
        self.rules.clear();
    }

    /// Returns an the entry for the given head within the set of rules.
    pub fn entry(&mut self, head: Head<Q, S>) -> hash_map::Entry<Head<Q, S>, Tail<Q, S>>
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
    {
        self.rules.entry(head)
    }
    /// Returns an immutable reference to the tail of the rule for the given head; returns none
    /// if the head is not found.
    pub fn get_head(&self, head: &Head<Q, S>) -> Option<&Tail<Q, S>>
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
    {
        self.rules.get(head)
    }

    /// Returns a mutable reference to the tail of the rule for the given head; returns none if
    /// the head is not found.
    pub fn get_mut(&mut self, head: &Head<Q, S>) -> Option<&mut Tail<Q, S>>
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
    {
        self.rules.get_mut(head)
    }

    /// Returns the tail of the rule for the given head; returns none if the head is not found
    /// within the set of rules.
    pub fn get_ref(&self, head: &Head<Q, S>) -> Option<Tail<&Q, &S>>
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
    {
        self.get_head(head).map(|tail| tail.to_ref())
    }

    /// Inserts a new rule into the set of rules.
    pub fn insert(&mut self, head: Head<Q, S>, tail: Tail<Q, S>)
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
    {
        self.rules.insert(head, tail);
    }

    /// Inserts a new rule into the set of rules.
    pub fn insert_rule(&mut self, rule: Rule<Q, S>)
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
    {
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

    /// Returns a mutable reference to the tail of the rule for the given head; inserts the
    /// tail if the head is not found.
    pub fn or_insert(&mut self, head: Head<Q, S>, tail: Tail<Q, S>) -> &mut Tail<Q, S>
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
    {
        self.rules.entry(head).or_insert(tail)
    }

    /// Returns a mutable reference to the tail of the rule for the given head; inserts the
    /// tail if the head is not found.
    pub fn or_insert_with<F>(&mut self, head: Head<Q, S>, f: F) -> &mut Tail<Q, S>
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
        F: FnOnce() -> Tail<Q, S>,
    {
        self.rules.entry(head).or_insert_with(f)
    }

    /// Returns a mutable reference to the tail of the rule for the given head; inserts the
    /// default tail if the head is not found.
    pub fn or_insert_default(&mut self, head: Head<Q, S>) -> &mut Tail<Q, S>
    where
        Q: Default + Eq + core::hash::Hash,
        S: Default + Eq + core::hash::Hash,
    {
        self.or_insert(head, Tail::default())
    }

    /// Removes a rule from the set of rules.
    pub fn remove(&mut self, head: &Head<Q, S>) -> Option<Tail<Q, S>>
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
    {
        self.rules.remove(head)
    }
}

#[cfg(feature = "serde")]
impl<'a, Q, S> serde::Deserialize<'a> for RuleMap<Q, S>
where
    Q: Eq + core::hash::Hash + serde::Deserialize<'a>,
    S: Eq + core::hash::Hash + serde::Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        struct RuleSetVisitor<Q, S> {
            _phantom: core::marker::PhantomData<(Q, S)>,
        }

        impl<'a, Q, S> serde::de::Visitor<'a> for RuleSetVisitor<Q, S>
        where
            Q: Eq + core::hash::Hash + serde::Deserialize<'a>,
            S: Eq + core::hash::Hash + serde::Deserialize<'a>,
        {
            type Value = RuleMap<Q, S>;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a map of rules")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'a>,
            {
                let mut rules = HashMap::with_capacity(map.size_hint().unwrap_or(0));
                while let Some((head, tail)) = map.next_entry()? {
                    rules.insert(head, tail);
                }
                Ok(RuleMap {
                    initial_state: None,
                    rules,
                })
            }
        }

        deserializer.deserialize_map(RuleSetVisitor {
            _phantom: core::marker::PhantomData,
        })
    }
}

impl<Q, S> core::iter::Extend<(Head<Q, S>, Tail<Q, S>)> for RuleMap<Q, S>
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

impl<Q, S> core::iter::Extend<Rule<Q, S>> for RuleMap<Q, S>
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

impl<Q, S> core::ops::Index<Head<Q, S>> for RuleMap<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: Eq + core::hash::Hash,
{
    type Output = Tail<Q, S>;

    fn index(&self, head: Head<Q, S>) -> &Self::Output {
        &self.rules[&head]
    }
}

impl<Q, S> core::iter::FromIterator<(Head<Q, S>, Tail<Q, S>)> for RuleMap<Q, S>
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

impl<Q, S> core::iter::FromIterator<Rule<Q, S>> for RuleMap<Q, S>
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

impl<Q, S> core::iter::IntoIterator for RuleMap<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: Eq + core::hash::Hash,
{
    type Item = (Head<Q, S>, Tail<Q, S>);
    type IntoIter = std::collections::hash_map::IntoIter<Head<Q, S>, Tail<Q, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}

impl<'a, Q, S> core::iter::IntoIterator for &'a RuleMap<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: Eq + core::hash::Hash,
{
    type Item = (&'a Head<Q, S>, &'a Tail<Q, S>);
    type IntoIter = std::collections::hash_map::Iter<'a, Head<Q, S>, Tail<Q, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.iter()
    }
}

impl<'a, Q, S> core::iter::IntoIterator for &'a mut RuleMap<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: Eq + core::hash::Hash,
{
    type Item = (&'a Head<Q, S>, &'a mut Tail<Q, S>);
    type IntoIter = std::collections::hash_map::IterMut<'a, Head<Q, S>, Tail<Q, S>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.iter_mut()
    }
}
