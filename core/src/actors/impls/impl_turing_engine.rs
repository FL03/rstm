/*
    Appellation: impl_turing_engine <module>
    Created At: 2025.08.31:14:48:57
    Contrib: @FL03
*/
use crate::actors::tmh::TMH;
use crate::actors::{Engine, RawEngine, TuringEngine};
use crate::program::Program;
use crate::{Head, Read, Symbol, Tail};
use rstm_state::{Halt, Haltable, RawState, State};
use rstm_traits::Handle;

impl<'a, Q, A> TuringEngine<'a, Q, A>
where
    Q: RawState,
{
    /// returns true if the driver is in a halted state
    pub fn is_halted(&self) -> bool
    where
        Q: Haltable + 'static,
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
    /// Reads the current symbol at the head of the tape
    pub fn load_head(&self) -> crate::Result<Head<&Q, &A>> {
        self.driver().get_head()
    }
    pub fn read(&mut self) -> crate::Result<&A>
    where
        A: Clone,
    {
        self.driver
            .read(&mut self.output)
            .expect("Failed to read from the tape");
        Ok(&self.output[self.driver().current_position()])
    }
    /// Reads the current symbol at the head of the tape
    pub fn read_uninit(&self) -> Head<&Q, core::mem::MaybeUninit<&A>> {
        if let Ok(Head { state, symbol }) = self.load_head() {
            Head {
                state,
                symbol: core::mem::MaybeUninit::new(symbol),
            }
        } else {
            Head {
                state: self.current_state().view(),
                symbol: core::mem::MaybeUninit::uninit(),
            }
        }
    }
    /// runs the program until termination (i.e., a halt state is reached, an error occurs, etc.)
    pub fn run(&mut self) -> crate::Result<()>
    where
        Q: 'static + Haltable + RawState + Clone + PartialEq,
        A: Symbol,
    {
        // check for a program
        if !self.has_program() {
            #[cfg(feature = "tracing")]
            tracing::error!("No program loaded; cannot execute step.");
            return Err(crate::Error::NoProgram);
        }
        #[cfg(feature = "tracing")]
        tracing::info!("Running the program...");
        while let Some(_h) = self.step()? {
            // #[cfg(feature = "tracing")]
            // tracing::info!(
            //     "Output after step #{i}: {tape:?}",
            //     i = self.cycles,
            //     tape = self.output,
            // );
            if self.driver.is_halted() {
                #[cfg(feature = "tracing")]
                tracing::info!(
                    "The engine has halted after {} steps; terminating the execution of the program.",
                    self.cycles
                );
                break;
            }
        }
        Ok(())
    }
    /// execute a single step of the program
    #[cfg_attr(
        feature = "tracing", 
        tracing::instrument(skip_all, fields(step = %self.cycles), name = "step", target = "TuringEngine", level = "trace")
    )]
    pub fn step(&mut self) -> crate::Result<Option<Head<Q, A>>>
    where
        Q: 'static + Haltable + RawState + Clone + PartialEq,
        A: Symbol,
    {
        // increment the steps
        self.next_cycle();
        #[cfg(feature = "tracing")]
        tracing::info!("{tape:?}", tape = self.driver());
        // check if the actor is halted
        if self.is_halted() {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "A halted stated was detected; terminating the execution of the program..."
            );
            return Ok(None);
        }
        // read the tape
        let Head { state, symbol } = self.load_head()?;
        // execute the program
        let tail = self
            .program()?
            .find_tail(state, symbol)
            .ok_or(crate::Error::NoRuleFound)?
            .clone();
        // process the instruction
        let next = tail.clone().into_head();
        // process the instruction
        let _prev = self.handle(tail);
        Ok(Some(next))
    }
}

#[allow(dead_code)]
/// This implementation is for any private methods used internally by the engine
impl<'a, Q, A> TuringEngine<'a, Q, A>
where
    Q: RawState,
{
    /// increments the current epoch by a single unit
    pub(crate) const fn next_cycle(&mut self) {
        self.cycles += 1;
    }
}

impl<'a, Q, A, X, Y> Handle<X> for TuringEngine<'a, Q, A>
where
    Q: RawState,
    TMH<Q, A>: Handle<X, Output = Y>,
{
    type Output = Y;

    fn handle(&mut self, args: X) -> Self::Output {
        self.driver_mut().handle(args)
    }
}

impl<'a, Q, A> RawEngine<Q, A> for TuringEngine<'a, Q, A>
where
    Q: RawState,
{
    type Driver = TMH<Q, A>;

    seal!();
}

impl<'a, Q, S> Engine<Q, S> for TuringEngine<'a, Q, S>
where
    Q: 'static + Haltable + RawState + Clone + PartialEq,
    S: Symbol,
{
    fn load(&mut self, program: Program<Q, S>) {
        self.program = Some(program);
    }

    fn run(&mut self) -> Result<(), crate::Error> {
        TuringEngine::run(self)
    }
}

impl<'a, Q, S> Iterator for TuringEngine<'a, Q, S>
where
    Q: 'static + Haltable + RawState + Clone + PartialEq,
    S: Symbol,
{
    type Item = Head<Q, S>;

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
