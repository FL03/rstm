/*
    Appellation: impl_turing_engine <module>
    Created At: 2025.08.31:14:48:57
    Contrib: @FL03
*/
use crate::actors::{Engine, RawEngine, TMH, TMHEngine};
use crate::error::Error;
use crate::programs::Program;
use crate::{Head, Read, Symbolic, Tail};
use rstm_state::{Halting, HaltingState, RawState, State};
use rstm_traits::Handle;

impl<'a, Q, A> TMHEngine<'a, Q, A>
where
    Q: RawState,
{
    /// returns the length of the input tape
    pub const fn len_input(&self) -> usize {
        self.driver().len()
    }
    pub fn current_position(&self) -> usize {
        self.driver().current_position()
    }
    /// returns true if the driver is in a halted state
    pub fn is_halted(&self) -> bool
    where
        Q: Halting,
    {
        self.driver().is_halted()
    }
    /// returns the tail associated with the head that is equal to the given state and symbol
    pub fn find_tail<K>(&self, state: State<&Q>, symbol: &A) -> Option<&Tail<Q, A>>
    where
        Q: Eq + core::hash::Hash,
        A: Eq + core::hash::Hash,
    {
        self.program.as_ref()?.find_tail(state, symbol)
    }
    /// a string representation of the driver's tape with the current head position highlighted
    /// in brackets. `0, 1, 0, [1], 1, 0, 0` for a radius of `3`.
    pub fn pretty_print(&self) -> String
    where
        A: core::fmt::Debug,
    {
        let mut out = String::new();
        let pos = self.current_position();
        let (a, b) = crate::get_range_around(pos, self.len_output(), 3);
        // print out the tape with the head position highlighted
        for (i, c) in self.output[a..=b].iter().enumerate() {
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
        let (a, b) = crate::get_range_around(pos, self.len_output(), 3);
        // print out the tape with the head position highlighted
        for (i, c) in self.output[a..=b].iter().enumerate() {
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
    /// Reads the current symbol at the head of the tape
    pub fn read_head(&self) -> crate::Result<Head<&Q, &A>> {
        self.output()
            .get(self.current_position())
            .map(|symbol| Head {
                state: self.driver().state().view(),
                symbol,
            })
            .ok_or_else(|| {
                #[cfg(feature = "tracing")]
                tracing::error!(
                    "[Index Error] the current position ({pos}) of the head is out of bounds...",
                    pos = self.current_position()
                );
                Error::IndexOutOfBounds {
                    index: self.current_position(),
                    len: self.output().len(),
                }
            })
    }
    /// read the current symbol at the head of the tape into the internal buffer
    pub fn read(&mut self, buf: &mut [A]) -> crate::Result<A>
    where
        A: Clone,
        TMH<Q, A>: Read<A, Error = Error, Output = usize>,
    {
        let pos = self.driver.read(&mut buf[..])?;
        Ok(buf[pos].clone())
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
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "run", target = "engine")
    )]
    /// runs the program until termination (i.e., a halt state is reached, an error occurs, etc.)
    pub fn run(&mut self) -> crate::Result<()>
    where
        Q: 'static + HaltingState + Clone + PartialEq,
        A: Symbolic,
    {
        let mut halted = false;
        // check for a program
        if !self.has_program() {
            #[cfg(feature = "tracing")]
            tracing::error!("No program loaded; cannot execute step.");
            return Err(Error::NoProgram);
        }
        #[cfg(feature = "tracing")]
        tracing::info!("engine loaded; beginning execution...");
        while let Some(_h) = self.step()? {
            #[cfg(feature = "tracing")]
            tracing::debug!("{}", self.print());
            // check for halting
            if self.driver.is_halted() {
                halted = true;
                break;
            }
        }
        if !halted {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "The engine terminated after {} steps without reaching a halt state.",
                self.cycles
            );
            Err(Error::ExitWithoutHalting)
        } else {
            #[cfg(feature = "tracing")]
            tracing::info!(
                "The engine has halted successfully after {} steps.",
                self.cycles
            );
            Ok(())
        }
    }
    /// execute a single step of the program
    #[cfg_attr(
        feature = "tracing", 
        tracing::instrument(skip_all, fields(step = %self.cycles), name = "step", target = "engine")
    )]
    pub fn step(&mut self) -> crate::Result<Option<Head<Q, A>>>
    where
        Q: 'static + RawState + Clone + PartialEq,
        A: Symbolic,
    {
        // if the output tape is empty, initialize it from the driver's tape
        if self.output().is_empty() {
            #[cfg(feature = "tracing")]
            tracing::warn! { "Output tape is empty; initializing from the input tape..." };
            let inputs = self.driver().tape().clone();
            self.extend_output(inputs);
        }
        // read the tape
        let Head { state, symbol } = self.read_head()?;
        // get a reference to the program
        if let Some(program) = self.program() {
            // use the program to find a tail for the current head
            let tail = program
                .find_tail(state, symbol)
                .ok_or(crate::Error::NoRuleFound)?
                .clone();
            // increment the steps
            self.next_cycle();
            // process the instruction
            let step = self.driver.head_mut().step(tail);
            // apply the step
            Ok(step.shift(&mut self.output))
        } else {
            #[cfg(feature = "tracing")]
            tracing::error!("No program loaded; cannot execute step.");
            Err(crate::Error::NoProgram)
        }
    }
}

impl<'a, Q, A> Read<A> for TMHEngine<'a, Q, A>
where
    A: Clone,
    Q: RawState,
    TMH<Q, A>: Read<A, Output = usize>,
{
    type Error = Error;
    type Output = A;

    fn read(&mut self, buf: &mut [A]) -> Result<Self::Output, Self::Error> {
        let pos = self
            .driver_mut()
            .read(&mut buf[..])
            .expect("reading from the TMH driver failed");
        Ok(buf[pos].clone())
    }
}

impl<'a, Q, A, X, Y> Handle<X> for TMHEngine<'a, Q, A>
where
    Q: RawState,
    TMH<Q, A>: Handle<X, Output = Y>,
{
    type Output = Y;

    fn handle(&mut self, args: X) -> Self::Output {
        self.driver_mut().handle(args)
    }
}

impl<'a, Q, A> RawEngine<Q, A> for TMHEngine<'a, Q, A>
where
    Q: RawState,
{
    type Driver = TMH<Q, A>;

    seal!();
}

impl<'a, Q, A> Engine<Q, A> for TMHEngine<'a, Q, A>
where
    Q: 'static + Halting + RawState + Clone + PartialEq,
    A: Symbolic,
{
    fn load(&mut self, program: Program<Q, A>) {
        self.program = Some(program);
    }

    fn run(&mut self) -> Result<(), crate::Error> {
        TMHEngine::run(self)
    }
}

impl<'a, Q, A> Iterator for TMHEngine<'a, Q, A>
where
    Q: 'static + Halting + RawState + Clone + PartialEq,
    A: Symbolic,
{
    type Item = Head<Q, A>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.step() {
            Ok(output) => match output {
                Some(h) => Some(h),
                None => {
                    #[cfg(feature = "tracing")]
                    tracing::info!("The engine has halted; terminating the iteration.");
                    None
                }
            },
            Err(_e) => {
                #[cfg(feature = "tracing")]
                tracing::error!("An error occurred during execution: {_e}");
                None
            }
        }
    }
}
