/*
    Appellation: instruction <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::builder::RuleBuilder;

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

mod builder {
    use super::*;

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
                head: Head {
                    state: self.state.expect("state is required"),
                    symbol: self.symbol.expect("symbol is required"),
                },
                tail: Tail {
                    direction: self.direction,
                    state: self.next_state.expect("next_state is required"),
                    symbol: self.write_symbol.expect("write_symbol is required"),
                },
            }
        }
    }
}
