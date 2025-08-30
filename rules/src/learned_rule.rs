/*
    Appellation: rules <module>
    Contrib: @FL03
*/
use crate::{Head, Rule, Tail};
use rstm::Direction;
use rstm_state::{RawState, State};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct LearnedRule<C = f32, Q = usize, S = usize>
where
    Q: RawState,
{
    pub confidence: C,
    pub head: Head<Q, S>,
    pub tail: Tail<Q, S>,
}

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
    pub const fn from_parts(
        state: State<Q>,
        symbol: S,
        direction: Direction,
        next_state: State<Q>,
        next_symbol: S,
        confidence: T,
    ) -> Self {
        // create a new head
        let head = Head::new(state, symbol);
        let tail = Tail::new(direction, next_state, next_symbol);
        Self::new(head, tail, confidence)
    }

    // pub fn from_rule(rule: rstm::Rule<Q, S>, confidence: C) -> Self {
    //     Self::new(rule.head, rule.tail, confidence)
    // }
    /// returns an immutable reference to the confidence of the rule
    pub const fn confidence(&self) -> &T {
        &self.confidence
    }
    /// returns a reference to the head of the rule
    pub fn head(&self) -> Head<&Q, &S> {
        self.head.view()
    }
    /// returns a mutable reference to the head of the rule
    pub fn head_mut(&mut self) -> Head<&mut Q, &mut S> {
        self.head.view_mut()
    }
    /// returns a reference to the tail of the rule
    pub fn tail(&self) -> Tail<&Q, &S> {
        self.tail.view()
    }
    /// returns a mutable reference to the tail of the rule
    pub fn tail_mut(&mut self) -> Tail<&mut Q, &mut S> {
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
