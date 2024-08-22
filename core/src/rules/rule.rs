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
    /// Returns an instance of the [Tail] whose elements are immutable references
    pub fn tail_ref(&self) -> Tail<&'_ Q, &'_ S> {
        self.tail().to_ref()
    }
    /// Returns the direction of the shift
    pub fn direction(&self) -> Direction {
        self.tail().direction()
    }
    /// Returns the current [State] of the system
    pub fn state(&self) -> State<&'_ Q> {
        self.head().state()
    }
    /// Returns the symbol of the [Head]
    pub const fn symbol(&self) -> &S {
        self.head().symbol()
    }
    /// Returns the next [Head] of the system
    pub fn next_head(&self) -> Head<&'_ Q, &'_ S> {
        self.tail().to_head_ref()
    }
    /// Consumes the current object and returns the next [Head] of the system
    pub fn into_next_head(self) -> Head<Q, S> {
        self.tail.into_head()
    }
    /// Returns the next [State] of the system
    pub fn next_state(&self) -> State<&'_ Q> {
        self.tail().state()
    }
    /// Returns the value which for which the current object will be replaced with
    pub const fn write_symbol(&self) -> &S {
        self.tail().symbol()
    }
    /// Consumes the current object and returns a 2-tuple consisting of the [Head] and [Tail]
    pub fn into_tuple(self) -> (Head<Q, S>, Tail<Q, S>) {
        (self.head, self.tail)
    }
}

impl<'a, Q, S> Rule<&'a Q, &'a S> {
    pub fn cloned(&self) -> Rule<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Rule {
            head: self.head.cloned(),
            tail: self.tail.cloned(),
        }
    }

    pub fn copied(&self) -> Rule<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Rule {
            head: self.head.copied(),
            tail: self.tail.copied(),
        }
    }
}

impl<Q, S> core::convert::AsRef<Head<Q, S>> for Rule<Q, S> {
    fn as_ref(&self) -> &Head<Q, S> {
        self.head()
    }
}

impl<Q, S> core::convert::AsRef<Tail<Q, S>> for Rule<Q, S> {
    fn as_ref(&self) -> &Tail<Q, S> {
        self.tail()
    }
}

impl<Q, S> core::convert::AsMut<Head<Q, S>> for Rule<Q, S> {
    fn as_mut(&mut self) -> &mut Head<Q, S> {
        self.head_mut()
    }
}

impl<Q, S> core::convert::AsMut<Tail<Q, S>> for Rule<Q, S> {
    fn as_mut(&mut self) -> &mut Tail<Q, S> {
        self.tail_mut()
    }
}

impl<Q, S> core::borrow::Borrow<Head<Q, S>> for Rule<Q, S> {
    fn borrow(&self) -> &Head<Q, S> {
        self.head()
    }
}

impl<Q, S> core::borrow::Borrow<Tail<Q, S>> for Rule<Q, S> {
    fn borrow(&self) -> &Tail<Q, S> {
        self.tail()
    }
}

impl<Q, S> core::borrow::BorrowMut<Head<Q, S>> for Rule<Q, S> {
    fn borrow_mut(&mut self) -> &mut Head<Q, S> {
        self.head_mut()
    }
}

impl<Q, S> core::borrow::BorrowMut<Tail<Q, S>> for Rule<Q, S> {
    fn borrow_mut(&mut self) -> &mut Tail<Q, S> {
        self.tail_mut()
    }
}

impl<Q, S> From<(Head<Q, S>, Tail<Q, S>)> for Rule<Q, S> {
    fn from((head, tail): (Head<Q, S>, Tail<Q, S>)) -> Self {
        Self { head, tail }
    }
}

impl<Q, S> From<Rule<Q, S>> for (Head<Q, S>, Tail<Q, S>) {
    fn from(rule: Rule<Q, S>) -> Self {
        (rule.head, rule.tail)
    }
}

impl<Q, S> From<RuleBuilder<Q, S>> for Rule<Q, S> {
    fn from(builder: RuleBuilder<Q, S>) -> Self {
        builder.build()
    }
}
