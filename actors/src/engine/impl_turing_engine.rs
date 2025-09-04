/*
    Appellation: impl_turing_engine <module>
    Created At: 2025.08.31:14:48:57
    Contrib: @FL03
*/
use super::TuringEngine;
use crate::tmh::TMH;
use crate::traits::{Engine, Handle, RawEngine};
use rstm_core::{Head, Symbol, Tail};
use rstm_rules::Program;
use rstm_state::{HaltState, RawState, State};

impl<'a, Q, A> TuringEngine<'a, Q, A>
where
    Q: RawState,
{
    pub const fn new(driver: &'a mut TMH<Q, A>) -> Self {
        Self {
            driver,
            _inputs: Vec::new(),
            program: None,
            epoch: 0,
        }
    }
    /// consumes the instance to return another loaded up with the given program
    pub fn load_with(self, program: Program<Q, A>) -> Self {
        TuringEngine {
            program: Some(program),
            ..self
        }
    }
    /// returns a reference to the actor
    pub const fn driver(&self) -> &TMH<Q, A> {
        self.driver
    }
    /// returns a mutable reference to the actor
    pub const fn driver_mut(&mut self) -> &mut TMH<Q, A> {
        self.driver
    }
    #[doc(hidden)]
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
    pub const fn current_epoch(&self) -> usize {
        self.epoch
    }
    /// returns a mutable reference to the current steps
    pub const fn current_state(&self) -> &State<Q> {
        self.driver().state()
    }
    /// returns true if the engine has a program loaded
    pub const fn has_program(&self) -> bool {
        self.program.is_some()
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
    /// runs the program until termination (i.e., a halt state is reached, an error occurs, etc.)
    pub fn run(&mut self) -> crate::Result<()>
    where
        Q: 'static + HaltState + Clone + PartialEq,
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
            #[cfg(feature = "tracing")]
            tracing::info!(
                "Executing step ({i}) with a context of {_h:?}",
                i = self.current_epoch(),
            );
        }
        Ok(())
    }
}

#[allow(dead_code)]
/// This implementation is for any private methods used internally by the engine
impl<'a, Q, A> TuringEngine<'a, Q, A>
where
    Q: RawState,
{
    /// increments the current epoch by a single unit
    pub(crate) const fn next_epoch(&mut self) {
        self.epoch += 1;
    }
    /// execute a single step of the program
    pub(crate) fn step(&mut self) -> crate::Result<Option<Head<Q, A>>>
    where
        Q: 'static + HaltState + Clone + PartialEq,
        A: Symbol,
    {
        #[cfg(feature = "tracing")]
        tracing::info!("{tape:?}", tape = self.driver());
        // check if the actor is halted
        if self.driver.is_halted() {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "A halted stated was detected; terminating the execution of the program..."
            );
            return Ok(None);
        }
        // read the tape
        let head = if let Ok(cur) = self.read() {
            cur
        } else {
            Head {
                state: self.driver().state().view(),
                symbol: &<A>::default(),
            }
        };
        // execute the program
        if let Some(tail) = self
            .program()
            .and_then(|p| p.find_tail(head.state, head.symbol))
            .cloned()
        {
            // process the instruction
            let next = tail.clone().into_head();
            // process the instruction
            let _prev = self.handle(tail);
            // update the epoch
            self.next_epoch();
            // return the head
            Ok(Some(next))
        } else {
            #[cfg(feature = "tracing")]
            tracing::error!("No symbol found at {}", self.driver.position());
            Err(crate::Error::NoSymbolFound(self.current_epoch()))
        }
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
    Q: 'static + HaltState + Clone + PartialEq,
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
    Q: 'static + HaltState + Clone + PartialEq,
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
