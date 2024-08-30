/*
    Appellation: program <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(feature = "alloc")]
use super::Rule;
use crate::{Head, State, Tail};
use alloc::vec::{self, Vec};

type Ruleset<Q, S> = Vec<Rule<Q, S>>;
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Program<Q = String, S = char> {
    pub(crate) initial_state: Option<State<Q>>,
    pub(crate) rules: Vec<Rule<Q, S>>,
}

impl<Q, S> Program<Q, S> {
    pub fn new() -> Self {
        Self {
            initial_state: None,
            rules: Vec::new(),
        }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Rule<Q, S>>,
    {
        Self {
            initial_state: None,
            rules: Vec::from_iter(iter),
        }
    }

    pub fn from_state(initial_state: State<Q>) -> Self {
        Self {
            initial_state: Some(initial_state),
            rules: Vec::new(),
        }
    }

    pub fn with_initial_state(self, state: State<Q>) -> Self {
        Self {
            initial_state: Some(state),
            ..self
        }
    }

    pub fn with_rules<I>(self, instructions: I) -> Self
    where
        I: IntoIterator<Item = Rule<Q, S>>,
    {
        Self {
            rules: Vec::from_iter(instructions),
            ..self
        }
    }

    /// Returns an owned reference to the initial state of the program.
    pub fn initial_state(&self) -> Option<State<&'_ Q>> {
        self.initial_state.as_ref().map(|state| state.to_ref())
    }

    /// Returns a reference to the instructions.
    pub const fn instructions(&self) -> &Ruleset<Q, S> {
        &self.rules
    }

    /// Returns a mutable reference to the instructions.
    pub fn instructions_mut(&mut self) -> &mut Ruleset<Q, S> {
        &mut self.rules
    }

    /// Returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<Rule<Q, S>> {
        self.rules.iter()
    }

    /// Returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<Rule<Q, S>> {
        self.rules.iter_mut()
    }

    pub fn get(&self, State(state): State<&Q>, symbol: &S) -> Option<&Tail<Q, S>>
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

    /// Returns a collection of tails for a given head.
    pub fn get_head(&self, head: &Head<Q, S>) -> Option<&Tail<Q, S>>
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

    /// Returns a mutable collection of tails for a given head.
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

    /// Returns a collection of tails for a given head.
    pub fn get_ref(&self, head: Head<&'_ Q, &'_ S>) -> Option<Tail<&'_ Q, &'_ S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter().find_map(|i| {
            if i.head_ref() == head {
                Some(i.tail_ref())
            } else {
                None
            }
        })
    }
}

impl<Q, S> AsRef<[Rule<Q, S>]> for Program<Q, S> {
    fn as_ref(&self) -> &[Rule<Q, S>] {
        &self.rules
    }
}

impl<Q, S> AsMut<[Rule<Q, S>]> for Program<Q, S> {
    fn as_mut(&mut self) -> &mut [Rule<Q, S>] {
        &mut self.rules
    }
}

impl<Q, S> core::ops::Deref for Program<Q, S> {
    type Target = [Rule<Q, S>];

    fn deref(&self) -> &Self::Target {
        &self.rules
    }
}

impl<Q, S> core::ops::DerefMut for Program<Q, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rules
    }
}

impl<Q, S> core::ops::Index<Head<Q, S>> for Program<Q, S>
where
    Q: PartialEq,
    S: PartialEq,
{
    type Output = Tail<Q, S>;

    fn index(&self, index: Head<Q, S>) -> &Self::Output {
        self.get_head(&index).unwrap()
    }
}

impl<Q: Default, S> From<Ruleset<Q, S>> for Program<Q, S> {
    fn from(instructions: Ruleset<Q, S>) -> Self {
        Self {
            initial_state: Some(State::default()),
            rules: instructions,
        }
    }
}

impl<Q, S> Extend<Rule<Q, S>> for Program<Q, S> {
    fn extend<I: IntoIterator<Item = Rule<Q, S>>>(&mut self, iter: I) {
        self.rules.extend(iter)
    }
}

impl<Q, S> FromIterator<Rule<Q, S>> for Program<Q, S>
where
    Q: Default,
{
    fn from_iter<I: IntoIterator<Item = Rule<Q, S>>>(iter: I) -> Self {
        Self {
            initial_state: Some(State::default()),
            rules: Ruleset::from_iter(iter),
        }
    }
}

impl<Q, S> IntoIterator for Program<Q, S> {
    type Item = Rule<Q, S>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rules.into_iter()
    }
}
