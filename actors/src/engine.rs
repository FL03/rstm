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
pub struct TuringEngine<'a, Q, A>
where
    Q: RawState,
{
    /// the actor that will be executing the program
    pub(crate) driver: &'a mut TMH<Q, A>,
    /// the program being executed
    pub(crate) program: Option<Program<Q, A>>,
    /// the number of steps taken by the actor
    pub(crate) steps: usize,
    pub(crate) _inputs: Vec<A>,
}

impl<'a, Q, A> TuringEngine<'a, Q, A>
where
    Q: RawState,
{
    pub const fn new(driver: &'a mut TMH<Q, A>) -> Self {
        Self {
            driver,
            _inputs: Vec::new(),
            program: None,
            steps: 0,
        }
    }
    /// Load a program into the executor
    pub fn load(self, program: Program<Q, A>) -> Self {
        TuringEngine {
            program: Some(program),
            ..self
        }
    }
    /// returns a reference to the actor
    pub const fn driver(&self) -> &TMH<Q, A> {
        &self.driver
    }
    /// returns a mutable reference to the actor
    pub const fn driver_mut(&mut self) -> &mut TMH<Q, A> {
        &mut self.driver
    }
    /// returns a reference to the inputs
    pub const fn inputs(&self) -> &Vec<A> {
        &self._inputs
    }
    /// returns a reference to the program
    pub fn program(&self) -> Option<&Program<Q, A>> {
        self.program.as_ref()
    }
    /// returns a mutable reference to the program
    pub fn program_mut(&mut self) -> Option<&mut Program<Q, A>> {
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
    pub fn find_tail<K>(&self, state: State<&Q>, symbol: &A) -> Option<&Tail<Q, A>>
    where
        Q: Eq + core::hash::Hash,
        A: Eq + core::hash::Hash,
    {
        self.program()?.find_tail(state, symbol)
    }
    /// Reads the current symbol at the head of the tape
    pub fn read(&self) -> crate::Result<Head<&Q, &A>> {
        self.driver().read()
    }
    /// Reads the current symbol at the head of the tape
    pub fn read_uninit(&self) -> Head<&Q, core::mem::MaybeUninit<&A>> {
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
        A: Symbolic,
    {
        #[cfg(feature = "tracing")]
        tracing::info!("Running the program...");
        while let Some(_h) = self.next() {
            #[cfg(feature = "tracing")]
            tracing::info!("Executing step: {head:?}", head = _h);
        }
        Ok(())
    }

    fn _handle_tail(&mut self, tail: Tail<Q, A>) -> crate::Result<Head<Q, A>>
    where
        Q: RawState + Clone + PartialEq,
        A: Symbolic,
    {
        // process the instruction
        let next = tail.as_head().cloned();
        // process the instruction
        let _prev = self.handle(tail);
        // return the head
        Ok(next)
    }
}

impl<'a, D, Q, S> Handle<D> for TuringEngine<'a, Q, S>
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

impl<'a, Q, S> Engine<Q, S> for TuringEngine<'a, Q, S>
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

impl<'a, Q, S> Iterator for TuringEngine<'a, Q, S>
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
