/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Executor;
use crate::rules::Program;
use crate::{Direction, Error, Head, State, Tail};

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
    pub fn new(alpha: impl IntoIterator<Item = S>, State(state): State<Q>, symbol: usize) -> Self {
        Self {
            alpha: Vec::from_iter(alpha),
            head: Head {
                state: State(state),
                symbol,
            },
        }
    }
    /// Constructs a new [Actor] with the given state; assumes the tape is empty and the head
    /// is located at `0`.
    pub fn from_state(State(state): State<Q>) -> Self {
        Self {
            alpha: Vec::new(),
            head: Head {
                state: State(state),
                symbol: 0,
            },
        }
    }
    /// Consumes the current instance and returns a new instance with the given alphabet
    pub fn with_alpha<I>(self, alpha: I) -> Self
    where
        I: IntoIterator<Item = S>,
    {
        Self {
            alpha: Vec::from_iter(alpha),
            ..self
        }
    }
    /// Consumes the current instance and returns a new instance with the given head
    pub fn with_head(self, head: Head<Q, usize>) -> Self {
        Self { head, ..self }
    }
    /// Consumes the current instance and returns a new instance with the given position
    pub fn with_position(self, symbol: usize) -> Self {
        Self {
            head: Head {
                state: self.head.state,
                symbol,
            },
            ..self
        }
    }
    /// Consumes the current instance and returns a new instance with the given state
    pub fn with_state(self, State(state): State<Q>) -> Self {
        Self {
            head: Head {
                state: State(state),
                symbol: self.head.symbol,
            },
            ..self
        }
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
    /// Returns an instance of the [Head] with an immutable reference to the state's inner
    /// value
    pub fn head_ref(&self) -> Head<&Q, usize> {
        Head {
            state: self.head.state.to_ref(),
            symbol: self.head.symbol,
        }
    }
    /// Returns the current position of the head on the tape
    pub fn position(&self) -> usize {
        self.head.symbol
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
    /// Reads the current symbol at the head of the tape
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "read", target = "actor")
    )]
    pub fn read(&self) -> Result<Head<&Q, &S>, Error> {
        #[cfg(feature = "tracing")]
        tracing::trace!("Reading the tape...");
        self.alpha
            .get(self.position())
            .map(|symbol| Head {
                state: self.head.state(),
                symbol,
            })
            .ok_or(Error::index_out_of_bounds(self.position(), self.len()))
    }

    /// Writes the given symbol to the tape
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "write", target = "actor")
    )]
    pub fn write(&mut self, value: S) {
        #[cfg(feature = "tracing")]
        tracing::trace!("Writing to the tape...");
        let pos = self.position();

        if pos == usize::MAX {
            #[cfg(feature = "tracing")]
            tracing::trace!("Prepending to the tape...");
        } else if pos >= self.len() {
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
    pub(crate) fn handle(&mut self, direction: Direction, State(state): State<Q>, symbol: S) {
        #[cfg(feature = "tracing")]
        tracing::trace!("Transitioning the actor...");
        // write the symbol to the tape
        self.write(symbol);
        // update the head of the actor
        self.head = Head {
            state: State(state),
            symbol: direction.apply_unsigned(self.head.symbol),
        };
    }

    pub(crate) fn process(&mut self, rule: Tail<Q, S>) {
        self.handle(rule.direction, rule.state, rule.symbol);
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
