/*
    Appellation: tmh <module>
    Created At: 2025.08.31:00:16:37
    Contrib: @FL03
*/

mod impl_tmh;

use crate::engine::TuringEngine;
use rstm_core::{Direction, Head};
use rstm_rules::Program;
use rstm_state::{RawState, State};

/// The [`TMH`] is an implementation of a Turing Machine with a "moving head"; this behavior is
/// manifested here by using the current position of the head as its symbol, serving as a
/// mapping to a symbol on the tape. Every step taken by the machine will update the symbol of
/// the head, thus _moving_ it along the tape.
///
/// The implementation is one of the primary _drivers_ used by actors within the library. 
/// By itself, the driver is not particularly useful, however, when given some input and a 
/// program, it can be used to perform computations.
#[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd)]
#[repr(C)]
pub struct TMH<Q, A> {
    /// the head of the tape
    pub(crate) head: Head<Q, usize>,
    /// the memory of the actor
    pub(crate) tape: Vec<A>,
}

impl<Q, A> TMH<Q, A>
where
    Q: RawState,
{
    pub const fn new(state: Q, symbol: usize) -> Self {
        Self {
            head: Head::new(state, symbol),
            tape: Vec::new(),
        }
    }
    /// returns a new instance of the [`TMH`] using the given state and an empty tape
    /// with the head positioned at `0`
    pub fn from_state(state: Q) -> Self {
        Self::new(state, 0)
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
    pub fn set_state(&mut self, state: Q) -> &mut Self {
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
    /// returns an engine loaded with the given program and using the current instance as the 
    /// driver.
    /// 
    /// **Note**: The engine is a _lazy_ executor, meaning that the program will not be run 
    /// until the corresponding `.run()` method is invoked on the engine.
    pub fn execute(self, program: Program<Q, A>) -> TuringEngine<Q, A> {
        TuringEngine::new(self).load(program)
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
            .ok_or(
                rstm_core::Error::IndexOutOfBounds {
                    index: self.position(),
                    len: self.len(),
                }
                .into(),
            )
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
        tracing::instrument(skip_all, name = "step", target = "tmh")
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
