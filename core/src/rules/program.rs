/*
    Appellation: program <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Instruction;
use crate::{Head, State, Tail};
use std::vec;

type Ruleset<Q, S> = Vec<Instruction<Q, S>>;

// type Ruleset<Q, S> = std::collections::HashMap<Head<Q, S>, Tail<Q, S>>;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Program<Q = String, S = char> {
    pub initial_state: State<Q>,
    pub(crate) ruleset: Ruleset<Q, S>,
}

impl<Q, S> Program<Q, S> {
    pub fn new() -> Self
    where
        Q: Default,
    {
        Self {
            initial_state: State::default(),
            ruleset: Vec::new(),
        }
    }

    pub fn from_iter(instructions: impl IntoIterator<Item = Instruction<Q, S>>) -> Self
    where
        Q: Default,
    {
        Self {
            initial_state: State::default(),
            ruleset: Ruleset::from_iter(instructions),
        }
    }

    pub fn from_state(State(initial_state): State<Q>) -> Self {
        Self {
            initial_state: State(initial_state),
            ruleset: Ruleset::new(),
        }
    }
    ///
    pub fn with_initial_state(self, State(state): State<Q>) -> Self {
        Self {
            initial_state: State(state),
            ..self
        }
    }
    ///
    pub fn with_instructions(
        self,
        instructions: impl IntoIterator<Item = Instruction<Q, S>>,
    ) -> Self {
        Self {
            ruleset: Ruleset::from_iter(instructions),
            ..self
        }
    }
    /// Returns an owned reference to the initial state of the program.
    pub fn initial_state(&self) -> State<&'_ Q> {
        self.initial_state.view()
    }
    /// Returns a reference to the instructions.
    pub const fn instructions(&self) -> &Ruleset<Q, S> {
        &self.ruleset
    }
    /// Returns a mutable reference to the instructions.
    pub fn instructions_mut(&mut self) -> &mut Ruleset<Q, S> {
        &mut self.ruleset
    }
    /// Returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<Instruction<Q, S>> {
        self.ruleset.iter()
    }
    /// Returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<Instruction<Q, S>> {
        self.ruleset.iter_mut()
    }

    pub fn get(&self, State(state): State<&Q>, symbol: &S) -> Option<&Tail<Q, S>>
    where
        Q: PartialEq,
        S: PartialEq,
    {
        self.iter().find_map(|i| {
            if i.head_ref() == Head::new(State(state), symbol) {
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

    pub fn get_head_mut(&mut self, head: &Head<Q, S>) -> Option<&mut Tail<Q, S>>
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
    pub fn get_head_ref(&self, head: Head<&'_ Q, &'_ S>) -> Option<Tail<&'_ Q, &'_ S>>
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

impl<Q, S> AsRef<[Instruction<Q, S>]> for Program<Q, S> {
    fn as_ref(&self) -> &[Instruction<Q, S>] {
        &self.ruleset
    }
}

impl<Q, S> AsMut<[Instruction<Q, S>]> for Program<Q, S> {
    fn as_mut(&mut self) -> &mut [Instruction<Q, S>] {
        &mut self.ruleset
    }
}

impl<Q, S> core::ops::Deref for Program<Q, S> {
    type Target = [Instruction<Q, S>];

    fn deref(&self) -> &Self::Target {
        &self.ruleset
    }
}

impl<Q, S> core::ops::DerefMut for Program<Q, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ruleset
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
            initial_state: State::default(),
            ruleset: instructions,
        }
    }
}

impl<Q, S> Extend<Instruction<Q, S>> for Program<Q, S> {
    fn extend<I: IntoIterator<Item = Instruction<Q, S>>>(&mut self, iter: I) {
        self.ruleset.extend(iter)
    }
}

impl<Q, S> FromIterator<Instruction<Q, S>> for Program<Q, S>
where
    Q: Default,
{
    fn from_iter<I: IntoIterator<Item = Instruction<Q, S>>>(iter: I) -> Self {
        Self {
            initial_state: State::default(),
            ruleset: Ruleset::from_iter(iter),
        }
    }
}

impl<Q, S> IntoIterator for Program<Q, S> {
    type Item = Instruction<Q, S>;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.ruleset.into_iter()
    }
}
