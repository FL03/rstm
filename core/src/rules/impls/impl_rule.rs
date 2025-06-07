/*
    Appellation: impl_rule <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::rules::Rule;
use crate::state::RawState;
use crate::types::{Head, Tail};

impl<Q, S> core::convert::AsRef<Head<Q, S>> for Rule<Q, S>
where
    Q: RawState,
{
    fn as_ref(&self) -> &Head<Q, S> {
        self.head()
    }
}

impl<Q, S> core::convert::AsRef<Tail<Q, S>> for Rule<Q, S>
where
    Q: RawState,
{
    fn as_ref(&self) -> &Tail<Q, S> {
        self.tail()
    }
}

impl<Q, S> core::convert::AsMut<Head<Q, S>> for Rule<Q, S>
where
    Q: RawState,
{
    fn as_mut(&mut self) -> &mut Head<Q, S> {
        self.head_mut()
    }
}

impl<Q, S> core::convert::AsMut<Tail<Q, S>> for Rule<Q, S>
where
    Q: RawState,
{
    fn as_mut(&mut self) -> &mut Tail<Q, S> {
        self.tail_mut()
    }
}

impl<Q, S> core::borrow::Borrow<Head<Q, S>> for Rule<Q, S>
where
    Q: RawState,
{
    fn borrow(&self) -> &Head<Q, S> {
        self.head()
    }
}

impl<Q, S> core::borrow::Borrow<Tail<Q, S>> for Rule<Q, S>
where
    Q: RawState,
{
    fn borrow(&self) -> &Tail<Q, S> {
        self.tail()
    }
}

impl<Q, S> core::borrow::BorrowMut<Head<Q, S>> for Rule<Q, S>
where
    Q: RawState,
{
    fn borrow_mut(&mut self) -> &mut Head<Q, S> {
        self.head_mut()
    }
}

impl<Q, S> core::borrow::BorrowMut<Tail<Q, S>> for Rule<Q, S>
where
    Q: RawState,
{
    fn borrow_mut(&mut self) -> &mut Tail<Q, S> {
        self.tail_mut()
    }
}

impl<Q, S> PartialEq<(Head<Q, S>, Tail<Q, S>)> for Rule<Q, S>
where
    Q: RawState + PartialEq,
    S: PartialEq,
{
    fn eq(&self, other: &(Head<Q, S>, Tail<Q, S>)) -> bool {
        self.head == other.0 && self.tail == other.1
    }
}

impl<Q, S> PartialEq<Head<Q, S>> for Rule<Q, S>
where
    Q: RawState + PartialEq,
    S: PartialEq,
{
    fn eq(&self, other: &Head<Q, S>) -> bool {
        &self.head == other
    }
}

impl<Q, S> PartialEq<Tail<Q, S>> for Rule<Q, S>
where
    Q: RawState + PartialEq,
    S: PartialEq,
{
    fn eq(&self, other: &Tail<Q, S>) -> bool {
        &self.tail == other
    }
}

impl<Q, S> From<(Head<Q, S>, Tail<Q, S>)> for Rule<Q, S>
where
    Q: RawState,
{
    fn from((head, tail): (Head<Q, S>, Tail<Q, S>)) -> Self {
        Self { head, tail }
    }
}

impl<Q, S> From<Rule<Q, S>> for (Head<Q, S>, Tail<Q, S>)
where
    Q: RawState,
{
    fn from(rule: Rule<Q, S>) -> Self {
        (rule.head, rule.tail)
    }
}
