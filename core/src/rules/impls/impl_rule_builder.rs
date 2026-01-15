/*
    appellation: impl_rule_builder <module>
    authors: @FL03
*/
use crate::rules::{Rule, RuleBuilder};

use crate::{Direction, Head, Tail};
use rstm_state::{RawState, State};

impl<Q1, Q2, A, B> RuleBuilder<Q1, A, Q2, B>
where
    Q1: RawState,
    Q2: RawState,
{
    /// initialize a new instance of the [`RuleBuilder`]
    pub const fn new() -> Self {
        Self {
            direction: Direction::Stay,
            state: None,
            symbol: None,
            next_state: None,
            write_symbol: None,
        }
    }
    /// configure the direction
    pub fn direction(self, direction: Direction) -> Self {
        Self { direction, ..self }
    }
    /// toggle the direction to the [`Left`](Direction::Left)
    pub fn left(self) -> Self {
        self.direction(Direction::Left)
    }
    /// toggle the direction to the [`Right`](Direction::Right)
    pub fn right(self) -> Self {
        self.direction(Direction::Right)
    }
    /// toggle the direction to the [`Stay`](Direction::Stay)
    pub fn stay(self) -> Self {
        self.direction(Direction::Stay)
    }
    /// configure the current state
    pub fn state<O>(self, state: State<O>) -> RuleBuilder<O, A, Q2, B>
    where
        O: RawState,
    {
        RuleBuilder {
            state: Some(state),
            symbol: self.symbol,
            direction: self.direction,
            next_state: self.next_state,
            write_symbol: self.write_symbol,
        }
    }
    /// configure the current symbol
    pub fn symbol<C>(self, symbol: C) -> RuleBuilder<Q1, C, Q2, B> {
        RuleBuilder {
            symbol: Some(symbol),
            state: self.state,
            direction: self.direction,
            next_state: self.next_state,
            write_symbol: self.write_symbol,
        }
    }
    /// configure the next state
    pub fn next_state<O>(self, state: State<O>) -> RuleBuilder<Q1, A, O, B>
    where
        O: RawState,
    {
        RuleBuilder {
            next_state: Some(state),
            state: self.state,
            symbol: self.symbol,
            direction: self.direction,
            write_symbol: self.write_symbol,
        }
    }
    /// configure the write symbol for the rule
    pub fn write_symbol<C>(self, write_symbol: C) -> RuleBuilder<Q1, A, Q2, C> {
        RuleBuilder {
            write_symbol: Some(write_symbol),
            state: self.state,
            symbol: self.symbol,
            direction: self.direction,
            next_state: self.next_state,
        }
    }
    /// consume the current instance to create a formal [`Rule`]
    #[inline]
    pub fn build(self) -> Rule<Q1, A, Q2, B> {
        Rule {
            head: Head {
                state: self.state.expect("state is required"),
                symbol: self.symbol.expect("symbol is required"),
            },
            tail: Tail {
                direction: self.direction,
                next_state: self.next_state.expect("next_state is required"),
                write_symbol: self.write_symbol.expect("write_symbol is required"),
            },
        }
    }
}

impl<Q1, A, Q2, B> From<RuleBuilder<Q1, A, Q2, B>> for Rule<Q1, A, Q2, B>
where
    Q1: RawState,
    Q2: RawState,
{
    fn from(builder: RuleBuilder<Q1, A, Q2, B>) -> Self {
        builder.build()
    }
}
