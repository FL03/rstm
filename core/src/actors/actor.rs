/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Executor;
use crate::rules::RuleSet;
use crate::{Direction, Error, Head, State};

/// An [Actor] is an implementation of a Turing machine with a moving head (TMH).
///
/// The model contains the following components:
///
/// - `alpha`: the input alphabet
/// - `head`: the head of the tape
#[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd)]
pub struct Actor<Q, A> {
    /// the head of the tape
    pub(crate) head: Head<Q, usize>,
    /// the input alphabet
    pub(crate) tape: Vec<A>,
}

impl<Q, A> Actor<Q, A> {
    pub fn new(alpha: impl IntoIterator<Item = A>, state: State<Q>, symbol: usize) -> Self {
        Self {
            head: Head { state, symbol },
            tape: Vec::from_iter(alpha),
        }
    }
    /// Constructs a new [Actor] with the given state; assumes the tape is empty and the head
    /// is located at `0`.
    pub fn from_state(state: State<Q>) -> Self {
        Self {
            head: Head { state, symbol: 0 },
            tape: Vec::new(),
        }
    }
    /// Consumes the current instance and returns a new instance with the given alphabet
    pub fn with_alpha<I>(self, alpha: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        Self {
            tape: Vec::from_iter(alpha),
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
                symbol,
                ..self.head
            },
            ..self
        }
    }
    /// Consumes the current instance and returns a new instance with the given state
    pub fn with_state(self, state: State<Q>) -> Self {
        Self {
            head: Head { state, ..self.head },
            ..self
        }
    }
    /// Returns an immutable reference to the tape, as a slice
    pub fn tape(&self) -> &[A] {
        &self.tape
    }
    /// Returns a mutable reference of the tape as a slice
    pub fn tape_mut(&mut self) -> &mut [A] {
        &mut self.tape
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
        self.head().symbol
    }
    /// Returns an instance of the state with an immutable reference to the inner value
    pub fn state(&self) -> State<&Q> {
        self.head().state()
    }
    /// Returns an instance of the state with a mutable reference to the inner value
    pub fn state_mut(&mut self) -> State<&mut Q> {
        self.head_mut().state_mut()
    }
    /// Executes the given program; the method is lazy, meaning it will not compute immediately
    /// but will return an [Executor] that is better suited for managing the runtime.
    pub fn execute(self, program: RuleSet<Q, A>) -> Executor<Q, A> {
        Executor::new(self, program)
    }
    /// Checks if the tape is empty
    pub fn is_empty(&self) -> bool {
        self.tape.is_empty()
    }
    /// Checks if the tape is halted
    pub fn is_halted(&self) -> bool
    where
        Q: 'static,
    {
        self.head().state.is_halt()
    }
    /// Returns the length of the tape
    #[inline]
    pub fn len(&self) -> usize {
        self.tape.len()
    }
    /// Reads the current symbol at the head of the tape
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "read", target = "actor")
    )]
    pub fn read(&self) -> Result<Head<&'_ Q, &'_ A>, Error> {
        #[cfg(feature = "tracing")]
        tracing::trace!("Reading the tape...");
        self.tape
            .get(self.position())
            .map(|symbol| Head {
                state: self.state(),
                symbol,
            })
            .ok_or(Error::index_out_of_bounds(self.position(), self.len()))
    }

    /// Writes the given symbol to the tape
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "write", target = "actor")
    )]
    pub fn write(&mut self, value: A) {
        #[cfg(feature = "tracing")]
        tracing::trace!("Writing to the tape...");
        let pos = self.position();

        if pos == usize::MAX {
            #[cfg(feature = "tracing")]
            tracing::trace!("Prepending to the tape...");
            // prepend to the tape
            self.tape.insert(0, value);
        } else if pos >= self.len() {
            #[cfg(feature = "tracing")]
            tracing::trace!("Appending to the tape...");
            // append to the tape
            self.tape.push(value);
        } else {
            self.tape[pos] = value;
        }
    }
    /// Performs a single step of the Turing machine; returns the previous head of the tape.
    /// Each step writes the given symbol to the tape, updates the state of the head, and moves
    /// the head by a single unit in the specified direction.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "handle", target = "actor")
    )]
    pub(crate) fn step(
        &mut self,
        direction: Direction,
        state: State<Q>,
        symbol: A,
    ) -> Head<Q, usize> {
        #[cfg(feature = "tracing")]
        tracing::trace!("Transitioning the actor...");
        // write the symbol to the tape
        self.write(symbol);
        // update the head of the actor
        self.head.replace(state, self.position() + direction)
    }
}

use crate::traits::transform::Handle;
use crate::Tail;

impl<Q, A> Handle<Tail<Q, A>> for Actor<Q, A> {
    type Output = Head<Q, usize>;

    fn handle(&mut self, args: Tail<Q, A>) -> Self::Output {
        self.step(args.direction, args.state, args.symbol)
    }
}

impl<Q, S> core::fmt::Debug for Actor<Q, S>
where
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for (i, c) in self.tape.iter().enumerate() {
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
        for (i, c) in self.tape.iter().enumerate() {
            if i == self.position() {
                write!(f, "[{c}]")?;
            } else {
                write!(f, "{c}")?;
            }
        }
        Ok(())
    }
}
