/*
    Appellation: workload <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

mod impl_rule_map;

use super::Rule;

use core::hash::Hash;
use rstm_core::{Head, Tail};
use rstm_state::{RawState, State};
use std::collections::hash_map::{self, HashMap};

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct RuleMap<Q, S>
where
    Q: RawState + Eq + Hash,
    S: Eq + Hash,
{
    pub(crate) initial_state: Option<State<Q>>,
    pub(crate) rules: HashMap<Head<Q, S>, Tail<Q, S>>,
}

impl<Q, S> RuleMap<Q, S>
where
    Q: RawState + Eq + Hash,
    S: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            initial_state: None,
            rules: HashMap::new(),
        }
    }
    /// returns a new instance of the [`RuleMap`] with the given initial state.
    pub fn from_state(initial_state: State<Q>) -> Self {
        Self {
            initial_state: Some(initial_state),
            rules: HashMap::new(),
        }
    }
    #[allow(clippy::should_implement_trait)]
    /// returns a new instance of the [`RuleMap`] composed from an iterator of head/tail pairs
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (Head<Q, S>, Tail<Q, S>)>,
    {
        Self {
            initial_state: None,
            rules: HashMap::from_iter(iter),
        }
    }

    pub fn from_rules<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = super::Rule<Q, S>>,
    {
        Self::from_iter(iter.into_iter().map(|Rule { head, tail }| (head, tail)))
    }
    /// consumes the current instance to create another with the given initial state.
    pub fn with_initial_state(self, state: State<Q>) -> Self {
        Self {
            initial_state: Some(state),
            ..self
        }
    }
    /// consumes the current instance to create another with the given instructions
    pub fn with_instructions(
        self,
        instructions: impl IntoIterator<Item = (Head<Q, S>, Tail<Q, S>)>,
    ) -> Self {
        Self {
            rules: HashMap::from_iter(instructions),
            ..self
        }
    }
    /// returns an instance of [State] which owns a reference to the interval value.
    pub fn initial_state(&self) -> Option<State<&'_ Q>> {
        self.initial_state.as_ref().map(|state| state.view())
    }
    /// returns an immutable reference to the set of rules.
    pub const fn rules(&self) -> &HashMap<Head<Q, S>, Tail<Q, S>> {
        &self.rules
    }
    /// returns a mutable reference to the set of rules.
    pub const fn rules_mut(&mut self) -> &mut HashMap<Head<Q, S>, Tail<Q, S>> {
        &mut self.rules
    }
    /// Clears the set of rules.
    pub fn clear(&mut self) {
        self.rules_mut().clear();
    }
    /// returns an the entry for the given head within the set of rules.
    pub fn rule(&mut self, head: Head<Q, S>) -> hash_map::Entry<'_, Head<Q, S>, Tail<Q, S>> {
        self.rules_mut().entry(head)
    }
    /// returns an immutable reference to the tail of the rule for the given head; returns none
    /// if the head is not found.
    pub fn get<K>(&self, head: &K) -> Option<&Tail<Q, S>>
    where
        K: Eq + Hash,
        Head<Q, S>: core::borrow::Borrow<K>,
    {
        self.rules().get(head)
    }
    /// returns a mutable reference to the tail of the rule for the given head; returns none if
    /// the head is not found.
    pub fn get_mut<K>(&mut self, head: &K) -> Option<&mut Tail<Q, S>>
    where
        K: Eq + Hash,
        Head<Q, S>: core::borrow::Borrow<K>,
    {
        self.rules_mut().get_mut(head)
    }
    /// returns the tail of the rule for the given head; returns none if the head is not found
    /// within the set of rules.
    pub fn get_tail_view<K>(&self, head: &K) -> Option<Tail<&Q, &S>>
    where
        K: Eq + Hash,
        Head<Q, S>: core::borrow::Borrow<K>,
    {
        self.get(head).map(|tail| tail.view())
    }
    /// insert a new rule into the set of rules.
    pub fn insert(&mut self, head: Head<Q, S>, tail: Tail<Q, S>)
    where
        Q: RawState + Eq + Hash,
        S: Eq + Hash,
    {
        self.rules_mut().insert(head, tail);
    }
    /// insert a new rule into the set of rules.
    pub fn insert_rule(&mut self, rule: Rule<Q, S>)
    where
        Q: RawState + Eq + Hash,
        S: Eq + Hash,
    {
        self.insert(rule.head, rule.tail);
    }
    /// returns true if the set of rules is empty.
    pub fn is_empty(&self) -> bool {
        self.rules().is_empty()
    }
    /// returns the number of rules in the set.
    pub fn len(&self) -> usize {
        self.rules().len()
    }
    /// returns a mutable reference to the tail of the rule for the given head; inserts the
    /// tail if the head is not found.
    pub fn or_insert(&mut self, head: Head<Q, S>, tail: Tail<Q, S>) -> &mut Tail<Q, S> {
        self.rule(head).or_insert(tail)
    }
    /// returns a mutable reference to the tail of the rule for the given head; inserts the
    /// tail if the head is not found.
    pub fn or_insert_with<F>(&mut self, head: Head<Q, S>, f: F) -> &mut Tail<Q, S>
    where
        F: FnOnce() -> Tail<Q, S>,
    {
        self.rule(head).or_insert_with(f)
    }
    /// returns a mutable reference to the tail of the rule for the given head; inserts the
    /// default tail if the head is not found.
    pub fn or_insert_default(&mut self, head: Head<Q, S>) -> &mut Tail<Q, S>
    where
        Q: Default,
        S: Default,
    {
        self.or_insert(head, Tail::default())
    }
    /// Removes a rule from the set of rules.
    pub fn remove(&mut self, head: &Head<Q, S>) -> Option<Tail<Q, S>> {
        self.rules_mut().remove(head)
    }
}

#[allow(deprecated)]
impl<Q, S> RuleMap<Q, S>
where
    Q: RawState + Eq + Hash,
    S: Eq + Hash,
{
    #[deprecated(since = "0.1.0", note = "use `get` instead, which is more concise.")]
    pub fn get_head<K>(&self, head: &K) -> Option<&Tail<Q, S>>
    where
        K: Eq + Hash,
        Head<Q, S>: core::borrow::Borrow<K>,
    {
        self.rules().get(head)
    }
}
