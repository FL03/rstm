/*
    Appellation: program <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Instruction;
use crate::{Head, State, Tail};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::collections::{btree_set as set, BTreeSet as Set};
#[cfg(feature = "std")]
use std::collections::{hash_set as set, HashSet as Set};

type RuleSet<Q, S> = Set<Instruction<Q, S>>;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Program<Q = String, S = char> {
    initial_state: State<Q>,
    instructions: RuleSet<Q, S>,
}

impl<Q, S> Program<Q, S> {
    pub fn new(State(initial_state): State<Q>) -> Self {
        Self {
            initial_state: State(initial_state),
            instructions: RuleSet::new(),
        }
    }
    ///
    pub fn with_initial_state(self, initial_state: State<Q>) -> Self {
        Self {
            initial_state,
            ..self
        }
    }

    pub fn with_instructions(self, instructions: RuleSet<Q, S>) -> Self {
        Self {
            instructions,
            ..self
        }
    }
    /// Returns an owned reference to the initial state of the program.
    pub const fn initial_state(&self) -> &State<Q> {
        &self.initial_state
    }
    /// Returns a reference to the instructions.
    pub const fn instructions(&self) -> &RuleSet<Q, S> {
        &self.instructions
    }
    /// Returns a mutable reference to the instructions.
    pub fn instructions_mut(&mut self) -> &mut RuleSet<Q, S> {
        &mut self.instructions
    }
    /// Returns an iterator over the elements.
    pub fn iter(&self) -> set::Iter<Instruction<Q, S>> {
        self.instructions.iter()
    }
    /// Returns an owned reference to the element(s) specified by the index.
    pub fn get(&self, head: &Head<Q, S>) -> Option<&Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter().find(|i| i.head() == head).map(|i| i.tail())
    }
}

#[cfg(feature = "std")]
impl<Q, S> Program<Q, S> {
    pub fn from_instructions(instructions: impl IntoIterator<Item = Instruction<Q, S>>) -> Self
    where
        Q: Default + Eq + core::hash::Hash,
        S: Default + Eq + core::hash::Hash,
    {
        Self {
            initial_state: State::default(),
            instructions: RuleSet::from_iter(instructions),
        }
    }
}
#[cfg(all(feature = "alloc", not(feature = "std")))]
impl<Q, S> Program<Q, S> {
    pub fn from_instructions(instructions: impl IntoIterator<Item = Instruction<Q, S>>) -> Self
    where
        Q: Default + Ord,
        S: Default + Ord,
    {
        Self {
            initial_state: State::default(),
            instructions: RuleSet::from_iter(instructions),
        }
    }

    pub fn iter(&self) -> set::Iter<Instruction<Q, S>> {
        self.instructions.iter()
    }

    pub fn iter_mut(&mut self) -> set::IterMut<Instruction<Q, S>> {
        self.instructions.iter_mut()
    }
}

#[cfg(feature = "std")]
impl<Q, S> IntoIterator for Program<Q, S> {
    type Item = Instruction<Q, S>;
    type IntoIter = set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.instructions.into_iter()
    }
}

#[cfg(all(feature = "alloc", not(feature = "std")))]
impl<Q, S> IntoIterator for Program<Q, S> {
    type Item = Instruction<Q, S>;
    type IntoIter = set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.instructions.into_iter()
    }
}
