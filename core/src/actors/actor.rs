/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::builder::ActorBuilder;

use super::Executor;
use crate::rules::Program;
use crate::{Direction, Head, State};

/// An [Actor] describes a Turing machine with a moving head (TMH).
///
/// [Actor]'s abstractly define actionable surfaces capable of executing a [Program].
#[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd)]
pub struct Actor<Q, S> {
    /// the input alphabet
    pub(crate) alpha: Vec<S>,
    /// the head of the tape
    pub(crate) head: Head<Q, usize>,
}

impl<Q, S> Actor<Q, S> {
    pub fn new(State(state): State<Q>) -> Self {
        Self {
            alpha: Vec::new(),
            head: Head {
                state: State(state),
                symbol: 0,
            },
        }
    }

    pub fn with_tape<I>(self, alpha: I) -> Self
    where
        I: IntoIterator<Item = S>,
    {
        Self {
            alpha: Vec::from_iter(alpha),
            ..self
        }
    }
    ///
    pub fn builder() -> ActorBuilder<Q, S> {
        ActorBuilder::new()
    }
    /// Returns an immutable reference to the tape alphabet as a slice
    pub fn alpha(&self) -> &[S] {
        &self.alpha
    }
    /// Returns an instance of the [Head] with an immutable reference to the state's inner
    /// value
    pub fn get_head_ref(&self) -> Head<&Q, usize> {
        Head {
            state: self.head.state.to_ref(),
            symbol: self.head.symbol,
        }
    }
    /// Returns an immutable reference to the head of the tape
    pub const fn head(&self) -> &Head<Q, usize> {
        &self.head
    }
    /// Returns a mutable reference to the head of the tape
    pub fn head_mut(&mut self) -> &mut Head<Q, usize> {
        &mut self.head
    }
    /// Returns an instance of the state with an immutable reference to the inner value
    pub fn state(&self) -> State<&Q> {
        self.head.state()
    }
    /// Returns an instance of the state with a mutable reference to the inner value
    pub fn state_mut(&mut self) -> State<&mut Q> {
        self.head.state_mut()
    }
    /// Executes the given program; the method is lazy, meaning it will not compute immediately
    /// but will return an [Executor] that is better suited for managing the runtime.
    pub fn execute(self, program: Program<Q, S>) -> Executor<Q, S> {
        Executor::new(self, program)
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
    /// Returns the current position of the head on the tape
    pub fn position(&self) -> usize {
        self.head.symbol
    }
    /// Reads the current symbol at the head of the tape
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "get", target = "actor")
    )]
    pub fn get(&self) -> Option<&S> {
        self.alpha().get(self.position())
    }

    pub fn get_head(&self) -> Option<Head<&Q, &S>> {
        self.get().map(|symbol| Head {
            state: self.head.state(),
            symbol,
        })
    }
    /// Reads the current symbol at the head of the tape
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "read", target = "actor")
    )]
    pub fn read(&self) -> Option<Head<&Q, &S>> {
        #[cfg(feature = "tracing")]
        tracing::trace!("Reading the tape...");
        self.alpha.get(self.position()).map(|symbol| Head {
            state: self.head.state(),
            symbol,
        })
    }

    /// Writes the given symbol to the tape
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "write", target = "actor")
    )]
    pub fn write(&mut self, value: S) {
        #[cfg(feature = "tracing")]
        tracing::trace!("Writing to the tape...");
        let pos = self.head.symbol;
        if pos >= self.len() {
            #[cfg(feature = "tracing")]
            tracing::trace!("Appending to the tape...");
            // append to the tape
            self.alpha.push(value);
        } else {
            self.alpha[pos] = value;
        }
    }
    /// Performs a single step of the Turing machine
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "handle", target = "actor")
    )]
    pub(crate) fn handle(
        &mut self,
        direction: Direction,
        State(next_state): State<Q>,
        symbol: S,
    ) {
        #[cfg(feature = "tracing")]
        tracing::trace!("Transitioning the actor...");
        // write the symbol to the tape
        self.write(symbol);
        // update the head of the actor
        self.head = Head {
            state: State(next_state),
            symbol: direction.apply(self.head.symbol),
        };
    }
    /// Performs a single step of the Turing machine
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "step", target = "actor")
    )]
    pub(crate) fn step(
        &mut self,
        direction: Direction,
        State(state): State<Q>,
        symbol: S,
    ) -> Option<Head<&Q, &S>>
    where
        S: Clone,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!("Transitioning the actor...");

        // write the symbol to the tape
        self.write(symbol);
        // update the head of the actor
        self.head = Head {
            state: State(state),
            symbol: direction.apply(self.head.symbol),
        };
        // read the tape
        self.read()
    }
}

impl<Q, S> core::fmt::Debug for Actor<Q, S>
where
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for (i, c) in self.alpha.iter().enumerate() {
            if i == self.position() {
                write!(f, "[{c:?}]")?;
            } else {
                write!(f, "{c:?}")?;
            }
        }
        Ok(())
    }
}

impl<Q, S> core::fmt::Display for Actor<Q, S>
where
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for (i, c) in self.alpha.iter().enumerate() {
            if i == self.position() {
                write!(f, "[{c}]")?;
            } else {
                write!(f, "{c}")?;
            }
        }
        Ok(())
    }
}

mod builder {
    use super::*;
    use core::iter::FromIterator;

    #[derive(Default)]
    pub struct ActorBuilder<Q, S> {
        alpha: Vec<S>,
        state: Option<State<Q>>,
        symbol: usize,
    }

    impl<Q, S> ActorBuilder<Q, S> {
        pub fn new() -> Self {
            Self {
                alpha: Vec::new(),
                state: None,
                symbol: 0,
            }
        }

        pub fn from_state(State(state): State<Q>) -> Self {
            Self {
                alpha: Vec::new(),
                state: Some(State(state)),
                symbol: 0,
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
            Self {
                state: Some(head.state),
                symbol: head.symbol,
                ..self
            }
        }

        pub fn state(self, State(state): State<Q>) -> Self {
            Self {
                state: Some(State(state)),
                ..self
            }
        }

        pub fn position(self, symbol: usize) -> Self {
            Self { symbol, ..self }
        }

        pub fn build(self) -> Actor<Q, S>
        where
            Q: Default,
        {
            let ActorBuilder {
                alpha,
                state,
                symbol,
            } = self;
            Actor {
                alpha,
                head: Head {
                    state: state.unwrap_or_default(),
                    symbol,
                },
            }
        }
    }
}
