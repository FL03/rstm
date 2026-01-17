/*
    Appellation: impl_tmh <module>
    Created At: 2025.08.31:00:34:06
    Contrib: @FL03
*/
#![cfg(feature = "alloc")]

use crate::actors::{EngineBase, TMH};
use crate::error::Error;
use crate::programs::Program;
use crate::{Direction, Head};
use alloc::string::String;
use alloc::vec::Vec;
use rstm_state::{Halting, RawState, State};

impl<Q, A> TMH<Q, A>
where
    Q: RawState,
{
    /// create a new instance of the [`TMH`] using the given state and tape.
    pub fn new<I>(state: Q, tape: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        Self {
            head: Head::new(state, 0),
            input: Vec::from_iter(tape),
        }
    }
    /// returns a new instance of the [`TMH`] using the given tape
    pub fn from_tape<I>(tape: I) -> Self
    where
        I: IntoIterator<Item = A>,
        Q: Default,
    {
        Self {
            head: Head::default(),
            input: Vec::from_iter(tape),
        }
    }
    /// returns a new instance of the [`TMH`] using the given state and an empty tape
    /// with the head positioned at `0`
    pub const fn from_state(state: Q) -> Self {
        Self {
            head: Head::new(state, 0),
            input: Vec::new(),
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
        &self.input
    }
    /// returns a mutable reference of the tape
    pub const fn tape_mut(&mut self) -> &mut Vec<A> {
        &mut self.input
    }
    /// returns the current position of the head on the tape
    pub const fn current_position(&self) -> usize {
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
    /// update the current head and return a mutable reference to the actor
    #[inline]
    pub fn set_head(&mut self, head: Head<Q, usize>) {
        self.head = head;
    }
    /// updates the current position of the head and returns a mutable reference to the actor
    #[inline]
    pub fn set_position(&mut self, symbol: usize) {
        self.head_mut().set_symbol(symbol);
    }
    /// updates the current state of the head and returns a mutable reference to the actor
    #[inline]
    pub fn set_state(&mut self, state: Q) {
        self.head_mut().set_state(state);
    }
    /// update the current tape and return a mutable reference to the actor
    #[inline]
    pub fn set_tape<I>(&mut self, tape: I)
    where
        I: IntoIterator<Item = A>,
    {
        self.input = Vec::from_iter(tape);
    }
    /// Consumes the current instance and returns a new instance with the given head
    #[inline]
    pub fn with_head(self, head: Head<Q, usize>) -> Self {
        Self { head, ..self }
    }
    /// Consumes the current instance and returns a new instance with the given position
    #[inline]
    pub fn with_position(self, symbol: usize) -> Self {
        Self {
            head: Head {
                symbol,
                ..self.head
            },
            ..self
        }
    }
    /// consumes the current instance to create another with the given state
    #[inline]
    pub fn with_state(self, state: State<Q>) -> Self {
        Self {
            head: Head { state, ..self.head },
            ..self
        }
    }
    /// consumes the current instance to create another with the given tape
    #[inline]
    pub fn with_tape<I>(self, alpha: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        Self {
            input: Vec::from_iter(alpha),
            ..self
        }
    }
    /// clears the tape
    pub fn clear(&mut self) {
        #[cfg(feature = "tracing")]
        tracing::trace!("clearing the tape...");
        self.tape_mut().clear();
    }
    /// extends the tape with elements from the given iterator
    #[inline]
    pub fn extend_tape<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = A>,
    {
        self.tape_mut().extend(iter)
    }
    /// returns the length of the tape
    pub const fn len(&self) -> usize {
        self.tape().len()
    }
    /// Checks if the tape is empty
    pub const fn is_empty(&self) -> bool {
        self.tape().is_empty()
    }
    /// Checks if the tape is halted
    pub fn is_halted(&self) -> bool
    where
        Q: Halting,
    {
        self.head().state().is_halted()
    }
    /// load the current instance and given program into a new instance of the
    /// [`TuringEngine`] implementation to directly manage the execution of the program.
    ///
    /// **Note**: The engine is a _lazy_ executor, meaning that the program will not be run
    /// until the corresponding `.run()` method is invoked on the engine.
    pub fn load(&mut self, program: Program<Q, A>) -> EngineBase<'_, Self, Q, A> {
        EngineBase::new(self, program)
    }
    /// returns the _current_ head of the actor, using the given directive to write some symbol
    /// onto the tape before shifting the head and updating its state.
    pub fn step(
        &mut self,
        direction: Direction,
        state: State<Q>,
        symbol: A,
    ) -> crate::Result<Head<Q, usize>> {
        let &position = self.head().symbol();
        #[cfg(feature = "tracing")]
        tracing::trace! { "Reacting to the current context of cell: {:?}", position }
        // write the symbol to the tape
        self.write(symbol)?;
        // update the head of the actor
        let prev = self.head_mut().replace(state, position + direction);

        Ok(prev)
    }
    /// returns a view of the current head containing immutable references to the state and
    /// symbol.
    ///
    /// ## Errors
    ///
    /// The method will return an error if the current position of the head is out of bounds;
    /// i.e., `i >= len()`.
    pub fn read_as_head(&self) -> crate::Result<Head<&'_ Q, &'_ A>> {
        self.tape()
            .get(self.current_position())
            .map(|symbol| Head {
                state: self.state().view(),
                symbol,
            })
            .ok_or_else(|| {
                #[cfg(feature = "tracing")]
                tracing::error!(
                    "[Index Error] the current position ({pos}) of the head is out of bounds...",
                    pos = self.current_position()
                );
                Error::IndexOutOfBounds {
                    idx: self.current_position(),
                    len: self.len(),
                }
            })
    }
    /// write a symbol to the tape at the current position of the head;
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "write", target = "tmh")
    )]
    pub fn write(&mut self, value: A) -> crate::Result<()> {
        let pos = self.current_position();

        if pos < self.len() {
            #[cfg(feature = "tracing")]
            tracing::trace! { "Updating the tape at {pos}" }
            self.input[pos] = value;
        } else if pos == self.len() {
            #[cfg(feature = "tracing")]
            tracing::trace! { "Extending the tape..." }
            // append to the tape
            self.tape_mut().push(value);
        } else {
            #[cfg(feature = "tracing")]
            tracing::trace! { "Prepending to the tape..." }
            // prepend to the tape
            self.tape_mut().insert(0, value);
        }
        Ok(())
    }
    /// a string representation of the driver's tape with the current head position highlighted
    /// in brackets. `0, 1, 0, [1], 1, 0, 0` for a radius of `3`.
    pub fn pretty_print(&self) -> String
    where
        A: core::fmt::Debug,
    {
        let mut cells = String::new();
        let pos = self.current_position();
        let (a, b) = crate::get_range_around(pos, self.len(), 3);
        // print out the tape with the head position highlighted
        for (i, c) in self.input[a..=b].iter().enumerate() {
            let idx = a + i;
            let cell = if pos == idx || (idx == b && pos == (idx + 1)) {
                format!("[[{c:?}]]")
            } else {
                format!("{c:?}")
            };
            cells.push_str(&cell);
        }
        cells
    }
    /// returns a string representation of the tape with the current head position highlighted
    /// in brackets.
    pub fn print(&self) -> String
    where
        A: core::fmt::Display,
    {
        let mut cells = Vec::new();
        let pos = self.current_position();
        let (a, b) = crate::get_range_around(pos, self.len(), 3);
        // print out the tape with the head position highlighted
        for (idx, c) in (a..=b).zip(self.input[a..=b].iter()) {
            let cell = if pos == idx || (idx == b && pos == (idx + 1)) {
                format! { "[{c}]" }
            } else {
                format! { "{c}" }
            };
            cells.push(cell);
        }
        cells.join("")
    }
}
