/*
    Appellation: instruction <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::builder::InstructionBuilder;

use crate::prelude::{Direction, State, StdHead, StdTail};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Instruction<Q = String, S = char> {
    pub head: StdHead<Q, S>,
    pub tail: StdTail<Q, S>,
}

impl<Q, S> Instruction<Q, S> {
    pub fn new() -> InstructionBuilder<Q, S> {
        InstructionBuilder::new()
    }

    pub const fn head(&self) -> &StdHead<Q, S> {
        &self.head
    }

    pub const fn tail(&self) -> &StdTail<Q, S> {
        &self.tail
    }

    pub fn direction(&self) -> Direction {
        self.tail().direction()
    }

    pub fn state(&self) -> State<&'_ Q> {
        self.head().get_state()
    }

    pub const fn symbol(&self) -> &S {
        self.head().symbol()
    }

    pub fn next_state(&self) -> State<&'_ Q> {
        self.tail().next_state()
    }

    pub const fn write_symbol(&self) -> &S {
        self.tail().write_symbol()
    }
}

mod builder {
    use super::*;

    pub struct InstructionBuilder<Q, S> {
        direction: Direction,
        state: Option<State<Q>>,
        symbol: Option<S>,
        next_state: Option<State<Q>>,
        write_symbol: Option<S>,
    }

    impl<Q, S> InstructionBuilder<Q, S> {
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

        pub fn build(self) -> Instruction<Q, S> {
            Instruction {
                head: StdHead {
                    state: self.state.expect("state is required"),
                    symbol: self.symbol.expect("symbol is required"),
                },
                tail: StdTail {
                    direction: self.direction,
                    state: self.next_state.expect("next_state is required"),
                    symbol: self.write_symbol.expect("write_symbol is required"),
                },
            }
        }
    }
}
