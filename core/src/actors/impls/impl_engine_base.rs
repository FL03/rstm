/*
    Appellation: impl_engine_base <module>
    Created At: 2026.01.17:19:50:55
    Contrib: @FL03
*/
use crate::actors::engine_base::EngineBase;

use crate::actors::RawDriver;
use crate::error::Error;
use crate::programs::Program;
use crate::rules::{Head, Tail};
use rstm_state::{Halting, RawState, State};
use rstm_traits::TryStep;

impl<D, Q, A> EngineBase<D, Q, A>
where
    D: RawDriver<Q, A>,
    Q: RawState,
{
    /// initialize a new instance of the engine using the default driver and given program
    pub fn from_program(program: Program<Q, A>) -> Self
    where
        D: Default,
    {
        Self {
            driver: D::default(),
            tape: Vec::new(),
            program: Some(program),
            cycles: 0,
        }
    }
    /// initialize a new engine using the given driver and program
    pub fn from_driver_with_input<I>(driver: D, input: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        Self {
            driver,
            tape: Vec::from_iter(input),
            program: None,
            cycles: 0,
        }
    }
    /// initialize a new instance of the engine from the given driver
    pub const fn from_driver(driver: D) -> Self {
        Self {
            driver,
            tape: Vec::new(),
            program: None,
            cycles: 0,
        }
    }
    /// load a new program into the engine and return a mutable reference to self
    pub fn load(self, program: Program<Q, A>) -> Self {
        Self {
            program: Some(program),
            ..self
        }
    }
    /// consumes the current instance to create another with the given tape input
    pub fn with_tape<I>(self, input: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        Self {
            tape: Vec::from_iter(input),
            ..self
        }
    }
    /// update the current program for the engine
    pub fn set_program(&mut self, program: Program<Q, A>) {
        self.program = Some(program);
    }
    #[inline]
    /// consumes the engine to create another with the given driver
    pub fn with_driver<D2>(self, driver: D2) -> EngineBase<D2, Q, A>
    where
        D2: RawDriver<Q, A>,
    {
        EngineBase {
            driver,
            program: self.program,
            cycles: self.cycles,
            tape: self.tape,
        }
    }
    #[inline]
    /// consumes the current instance to create another with the given program
    pub fn with_program(self, program: Program<Q, A>) -> Self {
        EngineBase {
            program: Some(program),
            ..self
        }
    }
    /// returns a copy of the total number of cycles, or steps, the engine has preformed
    pub const fn cycles(&self) -> usize {
        self.cycles
    }
    /// returns a reference to the actor
    pub const fn driver(&self) -> &D {
        &self.driver
    }
    /// returns a mutable reference to the actor
    pub const fn driver_mut(&mut self) -> &mut D {
        &mut self.driver
    }
    /// returns a reference to the program
    pub const fn program(&self) -> Option<&Program<Q, A>> {
        self.program.as_ref()
    }
    /// returns a mutable reference to the program
    pub const fn program_mut(&mut self) -> Option<&mut Program<Q, A>> {
        self.program.as_mut()
    }
    /// returns a reference to the output tape
    pub const fn tape(&self) -> &Vec<A> {
        &self.tape
    }
    /// returns a mutable reference to the output tape
    pub const fn tape_mut(&mut self) -> &mut Vec<A> {
        &mut self.tape
    }
    /// returns the current position of the driver
    pub fn current_position(&self) -> usize {
        self.driver().current_position()
    }
    /// returns a view of the current state of the driver
    pub fn current_state(&self) -> State<&Q> {
        self.driver().current_state()
    }
    /// returns the length of the output tape
    pub const fn len(&self) -> usize {
        self.tape().len()
    }
    /// returns true if the output tape is empty
    pub const fn is_empty(&self) -> bool {
        self.tape.is_empty() || self.program().is_none()
    }
    /// returns true if the engine has a program loaded
    pub const fn has_program(&self) -> bool {
        self.program.is_some()
    }
    /// extend the output tape with values from the given iterator
    pub fn extend_tape<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = A>,
    {
        self.tape_mut().extend(iter);
    }
    /// returns the tail associated with the head that is equal to the given state and symbol
    pub fn find_tail<K>(&self, state: State<&Q>, symbol: &A) -> Option<&Tail<Q, A>>
    where
        Q: Eq + core::hash::Hash,
        A: Eq + core::hash::Hash,
    {
        self.program.as_ref()?.find_tail(state, symbol)
    }
    /// increments the current epoch by a single unit indicating the end of a cycle or step
    pub const fn next_cycle(&mut self) {
        self.cycles += 1;
    }
    /// reset the engine by clearing the output tape, cycles, and program from the current
    /// instance
    pub fn reset(&mut self) {
        self.tape.clear();
        self.cycles = 0;
        self.program = None;
    }
    /// returns true if the driver is in a halted state
    pub fn is_halted(&self) -> bool
    where
        Q: Halting,
    {
        self.current_state().is_halted()
    }
    /// execute a single step of the engine
    pub fn step<Z>(&mut self) -> crate::Result<Z>
    where
        Self: TryStep<Output = Z, Error = crate::Error>,
    {
        self.try_step()
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "run", target = "engine")
    )]
    /// runs the program until termination (i.e., a halt state is reached, an error occurs, etc.)
    pub fn run(&mut self) -> crate::Result<()>
    where
        Q: Halting,
        Self: TryStep<Output = Head<Q, A>, Error = crate::Error>,
    {
        let mut halted = false;
        // check for a program
        if !self.has_program() {
            #[cfg(feature = "tracing")]
            tracing::error! { "unable to execute the workload without a program loaded." };
            return Err(Error::NoProgram);
        }
        #[cfg(feature = "tracing")]
        tracing::info! { "engine loaded; beginning execution..." };
        while let Ok(_h) = self.try_step() {
            // check for halting
            if self.is_halted() {
                halted = true;
                break;
            }
        }
        if !halted {
            #[cfg(feature = "tracing")]
            tracing::error!(
                "The engine terminated after {} steps before halting.",
                self.cycles
            );
            Err(Error::ExitWithoutHalting)
        } else {
            #[cfg(feature = "tracing")]
            tracing::info! { "successfully halted after {} steps.", self.cycles };
            Ok(())
        }
    }
    /// a string representation of the driver's tape with the current head position highlighted
    /// in brackets. `0, 1, 0, [1], 1, 0, 0` for a radius of `3`.
    pub fn pretty_print(&self) -> String
    where
        A: core::fmt::Debug,
    {
        let mut out = String::new();
        let pos = self.current_position();
        let (a, b) = crate::get_range_around(pos, self.len(), 3);
        // print out the tape with the head position highlighted
        for (i, c) in self.tape[a..=b].iter().enumerate() {
            let idx = a + i;
            let cell = if pos == idx || (idx == b && pos == (idx + 1)) {
                format!("[[{c:?}]]")
            } else {
                format!("{c:?}")
            };
            out.push_str(&cell);
        }
        out
    }
    /// returns a string representation of the tape with the current head position highlighted
    /// in brackets.
    pub fn print(&self) -> String
    where
        A: core::fmt::Display,
    {
        let mut out = String::new();
        let pos = self.current_position();
        let (a, b) = crate::get_range_around(pos, self.len(), 3);
        // print out the tape with the head position highlighted
        for (i, c) in self.tape[a..=b].iter().enumerate() {
            let idx = a + i;
            let cell = if pos == idx || (idx == b && pos == (idx + 1)) {
                format!("[[{c}]]")
            } else {
                format!("{c}")
            };
            out.push_str(&cell);
        }
        out
    }
    /// read the current symbol at the head of the tape into the internal buffer
    pub fn read(&self) -> crate::Result<&A> {
        let pos = self.current_position();
        Ok(&self.tape[pos])
    }
    /// read and return the current head of the machine
    pub fn read_head(&self) -> crate::Result<Head<&Q, &A>> {
        let symbol = self.read()?;
        Ok(Head {
            state: self.current_state(),
            symbol,
        })
    }
    /// Reads the current symbol at the head of the tape
    pub fn read_uninit(&self) -> Head<&Q, core::mem::MaybeUninit<&A>> {
        if let Ok(Head { state, symbol }) = self.read_head() {
            Head {
                state,
                symbol: core::mem::MaybeUninit::new(symbol),
            }
        } else {
            Head {
                state: self.current_state(),
                symbol: core::mem::MaybeUninit::uninit(),
            }
        }
    }
}
