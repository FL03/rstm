/*
    Appellation: impl_learned_rule <module>
    Created At: 2025.08.30:18:33:09
    Contrib: @FL03
*/
use super::LearnedRule;
use crate::rule::Rule;
use rstm_core::{Direction, Head, Tail};
use rstm_state::RawState;

impl<T, Q, S> LearnedRule<T, Q, S>
where
    Q: RawState,
{
    /// create a new [`LearnedRule`] using the given head, tail, and confidence
    pub const fn new(head: Head<Q, S>, tail: Tail<Q, S>, confidence: T) -> Self {
        Self {
            confidence,
            head,
            tail,
        }
    }
    /// returns a new instance using the given rule and confidence
    pub fn from_rule(rule: Rule<Q, S>, confidence: T) -> Self {
        Self::new(rule.head, rule.tail, confidence)
    }
    /// returns a new instance from its constituent parts
    pub const fn from_parts(
        state: Q,
        symbol: S,
        direction: Direction,
        next_state: Q,
        write_symbol: S,
        confidence: T,
    ) -> Self {
        // create a new head
        let head = Head::new(state, symbol);
        let tail = Tail::new(direction, next_state, write_symbol);
        Self::new(head, tail, confidence)
    }
    /// returns an immutable reference to the confidence of the rule
    pub const fn confidence(&self) -> &T {
        &self.confidence
    }
    /// returns a reference to the head of the rule
    pub const fn head(&self) -> Head<&Q, &S> {
        self.head.view()
    }
    /// returns a mutable reference to the head of the rule
    pub const fn head_mut(&mut self) -> Head<&mut Q, &mut S> {
        self.head.view_mut()
    }
    /// returns a reference to the tail of the rule
    pub const fn tail(&self) -> Tail<&Q, &S> {
        self.tail.view()
    }
    /// returns a mutable reference to the tail of the rule
    pub const fn tail_mut(&mut self) -> Tail<&mut Q, &mut S> {
        self.tail.view_mut()
    }
}

impl<Q, S, T> From<Rule<Q, S>> for LearnedRule<T, Q, S>
where
    Q: RawState,
    T: Default,
{
    fn from(rule: Rule<Q, S>) -> Self {
        Self::new(rule.head, rule.tail, <T>::default())
    }
}
