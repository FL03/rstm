/*
    Appellation: impl_turing_engine <module>
    Created At: 2025.08.31:14:48:57
    Contrib: @FL03
*/
use crate::actors::{Engine, RawEngine, TMH, TMHEngine};
use crate::programs::Program;
use crate::{Head, ReadInto, Symbolic, Tail};
use rstm_state::{IsHalted, RawState, State};
use rstm_traits::Handle;

impl<'a, Q, A> TMHEngine<'a, Q, A>
where
    Q: RawState,
{
    /// returns true if the driver is in a halted state
    pub fn is_halted(&self) -> bool
    where
        Q: IsHalted,
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
    pub fn get_head(&self) -> crate::Result<Head<&Q, &A>> {
        self.driver().get_head()
    }
    /// read the current symbol at the head of the tape into the internal buffer
    pub fn read(&mut self, buf: &mut [A]) -> crate::Result<A>
    where
        A: Clone,
        Self: ReadInto<A, Buf<A> = [A], Output = A>,
    {
        let pos = self
            .driver
            .read(&mut buf[..])
            .ok_or(crate::Error::NothingToRead)?;
        Ok(buf[pos].clone())
    }
    /// Reads the current symbol at the head of the tape
    pub fn read_uninit(&self) -> Head<&Q, core::mem::MaybeUninit<&A>> {
        if let Ok(Head { state, symbol }) = self.get_head() {
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
    /// runs the program until termination (i.e., a halt state is reached, an error occurs, etc.)
    pub fn run(&mut self) -> crate::Result<()>
    where
        Q: 'static + IsHalted + RawState + Clone + PartialEq,
        A: Symbolic,
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
        Q: 'static + IsHalted + RawState + Clone + PartialEq,
        A: Symbolic,
    {
        // if the output tape is empty, initialize it from the driver's tape
        if self.output.is_empty() {
            #[cfg(feature = "tracing")]
            tracing::warn! { "Output tape is empty; initializing from driver's tape..." };
            self.output.extend(self.driver().tape().clone());
        }
        self.output = self.driver().tape().to_vec();
        #[cfg(feature = "tracing")]
        tracing::trace!("{tape:?}", tape = self.driver());
        // check if the actor is halted
        if self.is_halted() {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "A halted stated was detected; terminating the execution of the program..."
            );
            return Ok(None);
        }
        // read the tape
        let Head { state, symbol } = self.get_head()?;
        // execute the program
        let tail = self
            .program()?
            .find_tail(state, symbol)
            .ok_or(crate::Error::NoRuleFound)?
            .clone();
        // increment the steps
        self.next_cycle();
        // process the instruction
        let step = self.driver.head_mut().step(tail);
        // apply the step
        Ok(step.shift(&mut self.output))
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
    Q: 'static + IsHalted + RawState + Clone + PartialEq,
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
    Q: 'static + IsHalted + RawState + Clone + PartialEq,
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
