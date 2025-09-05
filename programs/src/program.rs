/*
    Appellation: program <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#![cfg(feature = "alloc")]

#[allow(deprecated)]
mod impl_deprecated;
mod impl_program;

use crate::types::RuleVec;
use rstm_core::Rule;
use rstm_state::{RawState, State};

/// A [`Program`] defines an optional initial state along with a set of rules that dictate the
/// behavior of the system.
#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct Program<Q = String, A = char>
where
    Q: RawState,
{
    pub(crate) initial_state: Option<State<Q>>,
    pub(crate) rules: RuleVec<Q, A>,
}

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
}
