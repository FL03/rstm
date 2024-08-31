/*
    Appellation: rule <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::rules::Rule;
use crate::{Direction, State};

#[derive(Default)]
pub struct RuleBuilder<Q, S> {
    direction: Direction,
    state: Option<State<Q>>,
    symbol: Option<S>,
    next_state: Option<State<Q>>,
    write_symbol: Option<S>,
}

impl<Q, S> RuleBuilder<Q, S> {
    pub fn new() -> Self {
        Self {
            direction: Direction::Right,
            state: None,
            symbol: None,
            next_state: None,
            write_symbol: None,
        }
    }

    pub fn direction(self, direction: Direction) -> Self {
        Self { direction, ..self }
    }

    pub fn left(self) -> Self {
        self.direction(Direction::Left)
    }

    pub fn state(self, state: State<Q>) -> Self {
        Self {
            state: Some(state),
            ..self
        }
    }

    pub fn symbol(self, symbol: S) -> Self {
        Self {
            symbol: Some(symbol),
            ..self
        }
    }

    pub fn next_state(self, State(state): State<Q>) -> Self {
        Self {
            next_state: Some(State(state)),
            ..self
        }
    }

    pub fn write_symbol(self, write_symbol: S) -> Self {
        Self {
            write_symbol: Some(write_symbol),
            ..self
        }
    }

    pub fn build(self) -> Rule<Q, S> {
        Rule {
            head: crate::Head {
                state: self.state.expect("state is required"),
                symbol: self.symbol.expect("symbol is required"),
            },
            tail: crate::Tail {
                direction: self.direction,
                state: self.next_state.expect("next_state is required"),
                symbol: self.write_symbol.expect("write_symbol is required"),
            },
        }
    }
}

impl<Q, S> From<RuleBuilder<Q, S>> for Rule<Q, S> {
    fn from(builder: RuleBuilder<Q, S>) -> Self {
        builder.build()
    }
}