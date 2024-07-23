/*
    Appellation: program <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Instruction;
use crate::{Head, State, Tail};
use std::vec;

type RuleSet<Q, S> = Vec<Instruction<Q, S>>;

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

    pub fn from_instructions(instructions: impl IntoIterator<Item = Instruction<Q, S>>) -> Self
    where
        Q: Default,
    {
        Self {
            initial_state: State::default(),
            instructions: RuleSet::from_iter(instructions),
        }
    }
    ///
    pub fn with_initial_state(self, initial_state: State<Q>) -> Self {
        Self {
            initial_state,
            ..self
        }
    }

    pub fn with_instructions(
        self,
        instructions: impl IntoIterator<Item = Instruction<Q, S>>,
    ) -> Self {
        Self {
            instructions: RuleSet::from_iter(instructions),
            ..self
        }
    }
    /// Returns an owned reference to the element(s) specified by the index.
    pub fn get<I>(&self, idx: I) -> Option<&I::Output>
    where
        I: core::slice::SliceIndex<[Instruction<Q, S>]>,
    {
        self.instructions.get(idx)
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
    pub fn iter(&self) -> core::slice::Iter<Instruction<Q, S>> {
        self.instructions.iter()
    }
    /// Returns a collection of tails for a given head.
    pub fn get_head(&self, head: &Head<Q, S>) -> Vec<&Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter()
            .filter_map(|i| {
                if i.head() == head {
                    Some(i.tail())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<Q: Default, S> From<RuleSet<Q, S>> for Program<Q, S> {
    fn from(instructions: RuleSet<Q, S>) -> Self {
        Self {
            initial_state: State::default(),
            instructions,
        }
    }
}

impl<Q, S> FromIterator<Instruction<Q, S>> for Program<Q, S>
where
    Q: Default,
{
    fn from_iter<I: IntoIterator<Item = Instruction<Q, S>>>(iter: I) -> Self {
        Self {
            initial_state: State::default(),
            instructions: RuleSet::from_iter(iter),
        }
    }
}

impl<Q, S> IntoIterator for Program<Q, S> {
    type Item = Instruction<Q, S>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.instructions.into_iter()
    }
}
