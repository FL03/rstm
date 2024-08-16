/*
    Appellation: instruction <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::RuleBuilder;

use crate::prelude::{Direction, Head, State, Tail};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Rule<Q = String, S = char> {
    pub head: Head<Q, S>,
    pub tail: Tail<Q, S>,
}

impl<Q, S> Rule<Q, S> {
    pub fn new() -> RuleBuilder<Q, S> {
        RuleBuilder::new()
    }
    /// Returns an immutable reference to the [Head]
    pub const fn head(&self) -> &Head<Q, S> {
        &self.head
    }
    /// Returns a mutable reference to the [Head]
    pub fn head_mut(&mut self) -> &mut Head<Q, S> {
        &mut self.head
    }
    /// Returns an instance of the [Head] whose elements are immutable references
    pub fn head_ref(&self) -> Head<&'_ Q, &'_ S> {
        self.head().to_ref()
    }
    /// Returns an immutable reference to the [Tail] of the [Instruction]
    pub const fn tail(&self) -> &Tail<Q, S> {
        &self.tail
    }
    /// Returns a mutable reference to the [Tail] of the [Instruction]
    pub fn tail_mut(&mut self) -> &mut Tail<Q, S> {
        &mut self.tail
    }

    pub fn tail_ref(&self) -> Tail<&'_ Q, &'_ S> {
        self.tail().to_ref()
    }
    /// Returns the direction of the shift
    pub fn direction(&self) -> Direction {
        self.tail().direction()
    }
    /// Returns the current [state](State) of the [head](Head)
    pub fn state(&self) -> State<&'_ Q> {
        self.head().state()
    }
    /// Returns the current symbol of the [head](Head)
    pub const fn symbol(&self) -> &S {
        self.head().symbol()
    }
    pub fn next_head(&self) -> Head<&'_ Q, &'_ S> {
        self.tail().to_head_ref()
    }
    /// Returns the next [State] of the system
    pub fn next_state(&self) -> State<&'_ Q> {
        self.tail().next_state()
    }
    /// Returns the value which for which the current object will be replaced with
    pub const fn write_symbol(&self) -> &S {
        self.tail().write_symbol()
    }
}
