/*
    Appellation: exec <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Engine, Handle, TMH};
use rstm_core::{Head, Symbolic, Tail};
use rstm_rules::prelude::Program;
use rstm_state::{RawState, State};

/// The [`TuringEngine`] implementation is designed to handle the execution of a given program.
/// The exact nature of the engine is determined, in part, by the type of _driver_ it employs
///
pub struct TuringEngine<Q, S>
where
    Q: RawState,
{
    /// the actor that will be executing the program
    pub(crate) driver: TMH<Q, S>,
    /// the program being executed
    pub(crate) program: Option<Program<Q, S>>,
    /// the number of steps taken by the actor
    pub(crate) steps: usize,
}

impl<Q, S> TuringEngine<Q, S>
where
    Q: RawState,
{
    pub(crate) fn new(driver: TMH<Q, S>, program: Program<Q, S>) -> Self {
        Self {
            driver,
            program: Some(program),
            steps: 0,
        }
    }

    pub fn from_actor(driver: TMH<Q, S>) -> Self
    where
        Q: Default,
        S: Default,
    {
        Self {
            driver,
            program: None,
            steps: 0,
        }
    }
    /// Load a program into the executor
    pub fn load(self, program: Program<Q, S>) -> Self {
        TuringEngine {
            program: Some(program),
            ..self
        }
    }
    /// returns a reference to the actor
    pub const fn driver(&self) -> &TMH<Q, S> {
        &self.driver
    }
    /// returns a mutable reference to the actor
    pub const fn driver_mut(&mut self) -> &mut TMH<Q, S> {
        &mut self.driver
    }
    /// returns a reference to the program
    pub fn program(&self) -> Option<&Program<Q, S>> {
        self.program.as_ref()
    }
    /// returns a mutable reference to the program
    pub fn program_mut(&mut self) -> Option<&mut Program<Q, S>> {
        self.program.as_mut()
    }
    /// returns a copy of the current steps
    pub const fn steps(&self) -> usize {
        self.steps
    }
    /// returns a mutable reference to the current steps
    pub const fn current_state(&self) -> &State<Q> {
        self.driver().state()
    }
    /// returns the tail associated with the head that is equal to the given state and symbol
    pub fn find_tail<K>(&self, state: State<&Q>, symbol: &S) -> Option<&Tail<Q, S>>
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
    {
        self.program()?.find_tail(state, symbol)
    }
    /// Reads the current symbol at the head of the tape
    pub fn read(&self) -> crate::Result<Head<&Q, &S>> {
        self.driver().read()
    }
    /// Reads the current symbol at the head of the tape
    pub fn read_uninit(&self) -> Head<&Q, core::mem::MaybeUninit<&S>> {
        if let Ok(Head { state, symbol }) = self.read() {
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
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "run", target = "actor")
    )]
    pub fn run(&mut self) -> crate::Result<()>
    where
        Q: 'static + RawState + Clone + PartialEq,
        S: Symbolic,
    {
        #[cfg(feature = "tracing")]
        tracing::info!("Running the program...");
        while let Some(_h) = self.next() {
            #[cfg(feature = "tracing")]
            tracing::info!("Executing step: {head:?}", head = _h);
        }
        Ok(())
    }

    fn _handle_tail(&mut self, tail: Tail<Q, S>) -> crate::Result<Head<Q, S>>
    where
        Q: RawState + Clone + PartialEq,
        S: Symbolic,
    {
        // process the instruction
        let next = tail.as_head().cloned();
        // process the instruction
        let _prev = self.handle(tail);
        // return the head
        Ok(next)
    }
}

impl<D, Q, S> Handle<D> for TuringEngine<Q, S>
where
    Q: RawState + Clone + PartialEq,
    S: Symbolic,
    TMH<Q, S>: Handle<D>,
{
    type Output = <TMH<Q, S> as Handle<D>>::Output;

    fn handle(&mut self, args: D) -> Self::Output {
        self.driver_mut().handle(args)
    }
}

impl<Q, S> Engine<Q, S> for TuringEngine<Q, S>
where
    Q: 'static + RawState + Clone + PartialEq,
    S: Symbolic,
{
    fn load(&mut self, program: Program<Q, S>) {
        self.program = Some(program);
    }

    fn run(&mut self) -> Result<(), crate::Error> {
        TuringEngine::run(self)
    }
}

impl<Q, S> Iterator for TuringEngine<Q, S>
where
    Q: 'static + RawState + Clone + PartialEq,
    S: Symbolic,
{
    type Item = Head<Q, S>;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "next", target = "actor")
    )]
    fn next(&mut self) -> Option<Self::Item> {
        // increment the number of steps taken
        self.steps += 1;
        #[cfg(feature = "tracing")]
        tracing::info!("{tape:?}", tape = self.driver());
        // check if the actor is halted
        if self.driver.is_halted() {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "A halted stated was detected; terminating the execution of the program..."
            );
            return None;
        }
        // read the tape
        let head = if let Ok(cur) = self.read() {
            cur
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "[Index Error] the current position ({pos}) of the head is out of bounds, assuming the symbol to be its default value...",
                pos = self.driver().head().symbol()
            );
            Head {
                state: self.driver().state().view(),
                symbol: &S::default(),
            }
        };
        // execute the program
        if let Some(tail) = self.program()?.find_tail(head.state, head.symbol).cloned() {
            // process the instruction
            let next = tail.clone().into_head();
            // process the instruction
            let _prev = self.handle(tail);
            // return the head
            Some(next)
        } else {
            #[cfg(feature = "tracing")]
            tracing::error!("No symbol found at {}", self.driver.position());
            panic!("No symbol found at {}", self.driver.position());
        }
    }
}
