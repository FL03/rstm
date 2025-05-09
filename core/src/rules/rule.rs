/*
    Appellation: instruction <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::builder::RuleBuilder;
use crate::{Head, State, Tail};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Rule<Q = String, A = char> {
    pub head: Head<Q, A>,
    pub tail: Tail<Q, A>,
}

impl<Q, A> Rule<Q, A> {
    pub fn new() -> RuleBuilder<Q, A> {
        RuleBuilder::new()
    }
    /// Returns an immutable reference to the [Head]
    pub const fn head(&self) -> &Head<Q, A> {
        &self.head
    }
    /// Returns a mutable reference to the [Head]
    pub fn head_mut(&mut self) -> &mut Head<Q, A> {
        &mut self.head
    }
    /// Returns an instance of the [Head] whose elements are immutable references
    pub fn head_ref(&self) -> Head<&'_ Q, &'_ A> {
        self.head().to_ref()
    }
    /// Returns an immutable reference to the [Tail] of the [Instruction]
    pub const fn tail(&self) -> &Tail<Q, A> {
        &self.tail
    }
    /// Returns a mutable reference to the [Tail] of the [Instruction]
    pub fn tail_mut(&mut self) -> &mut Tail<Q, A> {
        &mut self.tail
    }
    /// Returns an instance of the [Tail] whose elements are immutable references
    pub fn tail_ref(&self) -> Tail<&'_ Q, &'_ A> {
        self.tail().to_ref()
    }
    /// Returns the direction of the shift
    pub fn direction(&self) -> crate::Direction {
        self.tail().direction()
    }
    /// Returns the current [State] of the system
    pub fn state(&self) -> State<&'_ Q> {
        self.head().state()
    }
    /// Returns the symbol of the [Head]
    pub const fn symbol(&self) -> &A {
        self.head().symbol()
    }
    /// Returns the next [Head] of the system
    pub fn next_head(&self) -> Head<&'_ Q, &'_ A> {
        self.tail().as_head()
    }
    /// Consumes the current object and returns the next [Head] of the system
    pub fn into_next_head(self) -> Head<Q, A> {
        self.tail.into_head()
    }
    /// Returns the next [State] of the system
    pub fn next_state(&self) -> State<&'_ Q> {
        self.tail().state()
    }
    /// Returns the value which for which the current object will be replaced with
    pub const fn write_symbol(&self) -> &A {
        self.tail().symbol()
    }
    /// Consumes the current object and returns a 2-tuple consisting of the [Head] and [Tail]
    pub fn into_tuple(self) -> (Head<Q, A>, Tail<Q, A>) {
        (self.head, self.tail)
    }
}

mod builder {
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
}
