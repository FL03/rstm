/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::builder::ActorBuilder;

use super::Executor;
use crate::rules::{Directive, Program};
use crate::{Head, State, Symbolic};

/// An [Actor] describes a Turing machine with a moving head (TMH).
pub struct Actor<Q, S> {
    /// the input alphabet
    pub(crate) alpha: Vec<S>,
    /// the head of the tape
    pub(crate) head: Head<Q, usize>,
}

impl<Q, S> Actor<Q, S> {
    pub fn new() -> ActorBuilder<Q, S>
    where
        Q: Default,
    {
        ActorBuilder::new()
    }

    pub fn from_state(State(state): State<Q>) -> ActorBuilder<Q, S> {
        ActorBuilder::from_state(State(state))
    }
    /// Returns an immutable reference to the tape alphabet as a slice
    pub fn alpha(&self) -> &[S] {
        &self.alpha
    }
    /// Returns an immutable reference to the head of the tape
    pub const fn head(&self) -> &Head<Q, usize> {
        &self.head
    }
    /// Returns a mutable reference to the head of the tape
    pub fn head_mut(&mut self) -> &mut Head<Q, usize> {
        &mut self.head
    }
    /// Returns the current state of the tape
    pub fn state(&self) -> State<&Q> {
        self.head.state()
    }
    /// Returns a mutable reference to the current state of the tape
    pub fn state_mut(&mut self) -> State<&mut Q> {
        self.head.state_mut()
    }
    /// Performs a single step of the Turing machine
    pub fn step<D>(&mut self, rule: &D) -> Head<&Q, &S>
    where
        D: Directive<Q, S>,
        S: Symbolic,
    {
        self.write(*rule.value());
        self.head.shift_inplace(rule.direction());
        self.read().expect("Invalid head position")
    }
    /// Executes the given program; the method is lazy, meaning it will not compute immediately
    /// but will return an [Executor] that is better suited for managing the runtime.
    pub fn exec(self, program: Program<Q, S>) -> Executor<Q, S> {
        Executor {
            actor: self,
            program,
        }
    }
    /// Checks if the tape is empty
    pub fn is_empty(&self) -> bool {
        self.alpha.is_empty()
    }
    /// Checks if the tape is halted
    pub fn is_halted(&self) -> bool
    where
        Q: 'static,
    {
        self.head.state.is_halt()
    }
    /// Returns the length of the tape
    #[inline]
    pub fn len(&self) -> usize {
        self.alpha.len()
    }
    /// Reads the current symbol at the head of the tape
    pub fn read(&self) -> Option<Head<&Q, &S>> {
        self.alpha.get(self.head.symbol).map(|symbol| Head {
            state: self.head.state.to_ref(),
            symbol,
        })
    }
    /// Writes the given symbol to the tape
    pub fn write(&mut self, symbol: S) {
        let head = self.head();
        if head.symbol < self.len() {
            self.alpha[self.head.symbol] = symbol;
        } else {
            // append to the tape
            self.alpha.push(symbol);
        }
    }
}

mod builder {
    use super::*;
    use std::iter::FromIterator;

    #[derive(Default)]
    pub struct ActorBuilder<Q, S> {
        alpha: Vec<S>,
        head: Head<Q, usize>,
    }

    impl<Q, S> ActorBuilder<Q, S> {
        pub fn new() -> Self
        where
            Q: Default,
        {
            Self {
                alpha: Vec::new(),
                head: Head::default(),
            }
        }

        pub fn from_state(State(state): State<Q>) -> Self {
            Self {
                alpha: Vec::new(),
                head: Head::new(State(state), 0),
            }
        }

        pub fn alpha<I>(self, alpha: I) -> Self
        where
            I: IntoIterator<Item = S>,
        {
            Self {
                alpha: Vec::from_iter(alpha),
                ..self
            }
        }

        pub fn head(self, head: Head<Q, usize>) -> Self {
            Self { head, ..self }
        }

        pub fn state(self, State(state): State<Q>) -> Self {
            Self {
                head: Head {
                    state: State(state),
                    ..self.head
                },
                ..self
            }
        }

        pub fn position(self, symbol: usize) -> Self {
            Self {
                head: Head {
                    symbol,
                    ..self.head
                },
                ..self
            }
        }

        pub fn build(self) -> Actor<Q, S>
        where
            Q: Default,
        {
            Actor {
                alpha: self.alpha,
                head: self.head,
            }
        }
    }
}
