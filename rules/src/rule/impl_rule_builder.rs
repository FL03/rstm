/*
    appellation: impl_rule_builder <module>
    authors: @FL03
*/
use super::{Rule, RuleBuilder};
use rstm::Direction;
use rstm_state::{RawState, State};

impl<Q, S> RuleBuilder<Q, S>
where
    Q: RawState,
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
    pub fn state(self, state: State<Q>) -> Self {
        Self {
            state: Some(state),
            ..self
        }
    }
    /// configure the current symbol
    pub fn symbol(self, symbol: S) -> Self {
        Self {
            symbol: Some(symbol),
            ..self
        }
    }
    /// configure the next state
    pub fn next_state(self, State(state): State<Q>) -> Self {
        Self {
            next_state: Some(State(state)),
            ..self
        }
    }
    /// configure the symbol to write
    pub fn write_symbol(self, write_symbol: S) -> Self {
        Self {
            write_symbol: Some(write_symbol),
            ..self
        }
    }
    /// consume the current instance to create a formal [`Rule`]
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

impl<Q, S> From<RuleBuilder<Q, S>> for Rule<Q, S>
where
    Q: RawState,
{
    fn from(builder: RuleBuilder<Q, S>) -> Self {
        builder.build()
    }
}
