/*
    Appellation: impl_learned_rule <module>
    Created At: 2025.08.30:18:33:09
    Contrib: @FL03
*/
use super::LearnedRule;
use crate::rule::Rule;
use crate::{Direction, Head, Tail};
use rstm_state::RawState;

impl<T, Q, S> LearnedRule<T, Q, S>
where
    Q: RawState,
{
    /// create a new [`LearnedRule`] using the given head, tail, and confidence
    pub const fn new(head: Head<Q, S>, tail: Tail<Q, S>, confidence: T) -> Self {
        Self {
            confidence,
            rule: Rule { head, tail },
        }
    }
    /// returns a new instance using the given rule and confidence
    pub const fn from_rule(rule: Rule<Q, S>, confidence: T) -> Self {
        Self { confidence, rule }
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
        Self {
            confidence,
            rule: Rule { head, tail },
        }
    }
    /// returns an immutable reference to the confidence of the rule
    pub const fn confidence(&self) -> &T {
        &self.confidence
    }
    /// returns a mutable reference to the confidence of the rule
    pub const fn confidence_mut(&mut self) -> &mut T {
        &mut self.confidence
    }
    /// returns an immutable reference to the learned rule
    pub const fn rule(&self) -> &Rule<Q, S> {
        &self.rule
    }
    /// returns a mutable reference to the learned rule
    pub const fn rule_mut(&mut self) -> &mut Rule<Q, S> {
        &mut self.rule
    }
    /// update the confidence level of the rule
    pub fn set_confidence(&mut self, confidence: T) {
        self.confidence = confidence;
    }
    /// update the rule
    pub fn set_rule(&mut self, rule: Rule<Q, S>) {
        self.rule = rule;
    }
    /// returns an immutable reference to the head of the rule
    pub const fn head(&self) -> &Head<Q, S> {
        self.rule().head()
    }
    /// returns a mutable reference to the head of the rule
    pub const fn head_mut(&mut self) -> &mut Head<Q, S> {
        self.rule_mut().head_mut()
    }
    /// returns an immutable reference to the tail of the rule
    pub const fn tail(&self) -> &Tail<Q, S> {
        self.rule().tail()
    }
    /// returns a mutable reference to the tail of the rule
    pub const fn tail_mut(&mut self) -> &mut Tail<Q, S> {
        self.rule_mut().tail_mut()
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

impl<Q, A, T> core::ops::Deref for LearnedRule<T, Q, A>
where
    Q: RawState,
{
    type Target = Rule<Q, A>;

    fn deref(&self) -> &Self::Target {
        &self.rule
    }
}
