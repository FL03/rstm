/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Executor, Handle};
use crate::rules::RuleSet;
use crate::state::{RawState, State};
use crate::{Direction, Error, Head, Symbolic, Tail};

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
    /// the memory of the actor
    pub(crate) tape: Vec<A>,
}

impl<Q, A> Actor<Q, A>
where
    Q: RawState,
{
    pub fn new(tape: Vec<A>, state: State<Q>, symbol: usize) -> Self {
        Self {
            head: Head { state, symbol },
            tape,
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
    /// returns an immutable reference to the head of the tape
    pub const fn head(&self) -> &Head<Q, usize> {
        &self.head
    }
    /// returns a mutable reference to the head of the tape
    pub const fn head_mut(&mut self) -> &mut Head<Q, usize> {
        &mut self.head
    }
    /// returns an immutable reference to the tape
    pub const fn tape(&self) -> &Vec<A> {
        &self.tape
    }
    /// returns a mutable reference of the tape
    pub const fn tape_mut(&mut self) -> &mut Vec<A> {
        &mut self.tape
    }
    /// update the current head and return a mutable reference to the actor
    pub fn set_head(&mut self, head: Head<Q, usize>) -> &mut Self {
        self.head = head;
        self
    }
    /// updates the current position of the head and returns a mutable reference to the actor
    pub fn set_position(&mut self, symbol: usize) -> &mut Self {
        self.head_mut().set_symbol(symbol);
        self
    }
    /// updates the current state of the head and returns a mutable reference to the actor
    pub fn set_state(&mut self, state: State<Q>) -> &mut Self {
        self.head_mut().set_state(state);
        self
    }
    /// update the current tape and return a mutable reference to the actor
    pub fn set_tape(&mut self, tape: Vec<A>) -> &mut Self {
        self.tape = tape;
        self
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
    /// Consumes the current instance and returns a new instance with the given alphabet
    pub fn with_tape<I>(self, alpha: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        Self {
            tape: Vec::from_iter(alpha),
            ..self
        }
    }
    /// returns an instance of the [Head] with an immutable reference to the state's inner
    /// value
    pub fn head_ref(&self) -> Head<&Q, usize> {
        Head {
            state: self.head.state.view(),
            symbol: self.head.symbol,
        }
    }
    /// returns the current position of the head on the tape
    pub const fn position(&self) -> usize {
        *self.head().symbol()
    }
    /// returns an instance of the state with an immutable reference to the inner value
    pub const fn state(&self) -> &State<Q> {
        self.head().state()
    }
    /// returns an instance of the state with a mutable reference to the inner value
    pub const fn state_mut(&mut self) -> &mut State<Q> {
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
        // self.head().state.is_halt()
        todo!("reconfigure the actor halting")
    }
    /// returns the length of the tape
    #[inline]
    pub fn len(&self) -> usize {
        self.tape.len()
    }
    /// Reads the current symbol at the head of the tape
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "read", target = "actor")
    )]
    pub fn read(&self) -> crate::Result<Head<&'_ Q, &'_ A>> {
        #[cfg(feature = "tracing")]
        tracing::trace!("Reading the tape...");
        self.tape
            .get(self.position())
            .map(|symbol| Head {
                state: self.state().view(),
                symbol,
            })
            .ok_or(Error::IndexOutOfBounds {
                index: self.position(),
                len: self.len(),
            })
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

impl<Q, S> Handle<(Direction, State<Q>, S)> for Actor<Q, S>
where
    Q: RawState + Clone + PartialEq,
    S: Symbolic,
{
    type Output = Head<Q, usize>;

    fn handle(&mut self, (direction, state, symbol): (Direction, State<Q>, S)) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, S> Handle<(Direction, Head<Q, S>)> for Actor<Q, S>
where
    Q: RawState + Clone + PartialEq,
    S: Symbolic,
{
    type Output = Head<Q, usize>;

    fn handle(
        &mut self,
        (direction, Head { state, symbol }): (Direction, Head<Q, S>),
    ) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, S> Handle<Tail<Q, S>> for Actor<Q, S>
where
    Q: RawState + Clone + PartialEq,
    S: Symbolic,
{
    type Output = Head<Q, usize>;

    fn handle(
        &mut self,
        Tail {
            direction,
            state,
            symbol,
        }: Tail<Q, S>,
    ) -> Self::Output {
        self.step(direction, state, symbol)
    }
}

impl<Q, S> core::fmt::Debug for Actor<Q, S>
where
    Q: RawState,
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
    Q: RawState,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for (i, c) in self.tape().iter().enumerate() {
            if i == self.position() {
                write!(f, "[{c}]")?;
            } else {
                write!(f, "{c}")?;
            }
        }
        Ok(())
    }
}
