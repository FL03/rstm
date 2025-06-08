/*
    Appellation: program <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Rule;
use crate::state::{RawState, State};
use crate::{Head, Tail};
use alloc::vec::Vec;

pub(crate) type Rules<Q, S> = alloc::vec::Vec<Rule<Q, S>>;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RuleSet<Q = String, S = char>
where
    Q: RawState,
{
    pub(crate) initial_state: Option<State<Q>>,
    pub(crate) rules: Rules<Q, S>,
}

impl<Q, S> RuleSet<Q, S>
where
    Q: RawState,
{
    pub fn new() -> Self {
        Self {
            initial_state: None,
            rules: Rules::new(),
        }
    }
    /// Create a new instance of the [Program] from the given rules.
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Rule<Q, S>>,
    {
        Self {
            initial_state: None,
            rules: Rules::from_iter(iter),
        }
    }
    /// Create a new instance of the [Program] using the given initial state.
    pub fn from_state(initial_state: State<Q>) -> Self {
        Self {
            initial_state: Some(initial_state),
            rules: Rules::new(),
        }
    }
    /// Returns an owned reference to the initial state of the program.
    pub fn initial_state(&self) -> Option<State<&'_ Q>> {
        self.initial_state.as_ref().map(|state| state.view())
    }
    /// Returns a reference to the instructions.
    pub const fn rules(&self) -> &Rules<Q, S> {
        &self.rules
    }
    /// Returns a mutable reference to the instructions.
    pub const fn rules_mut(&mut self) -> &mut Rules<Q, S> {
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
    pub fn iter(&self) -> core::slice::Iter<Rule<Q, S>> {
        self.rules().iter()
    }
    /// Returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<Rule<Q, S>> {
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

#[allow(deprecated)]
impl<Q, S> RuleSet<Q, S>
where
    Q: RawState + Default,
{
    #[deprecated(since = "0.0.8", note = "use `rules` instead")]
    pub const fn instructions(&self) -> &Rules<Q, S> {
        self.rules()
    }
    #[deprecated(since = "0.0.8", note = "use `rules_mut` instead")]
    pub fn instructions_mut(&mut self) -> &mut Rules<Q, S> {
        self.rules_mut()
    }
    #[deprecated(since = "0.0.8", note = "use `get` instead")]
    pub fn get_by_head(&self, head: &Head<Q, S>) -> Option<&Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.get(head)
    }
}
