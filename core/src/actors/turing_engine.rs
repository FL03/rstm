/*
    Appellation: engine <module>
    Created At: 2025.08.31:14:49:50
    Contrib: @FL03
*/
use super::{RawDriver, TMH};
use crate::programs::Program;
use alloc::vec::Vec;
use rstm_state::{RawState, State};

/// A type alias for a [`TuringEngine`] configured with a _moving head_ execution model, or
/// driver.
pub type TMHEngine<'a, Q, A> = TuringEngine<'a, TMH<Q, A>, Q, A>;

/// The [`TuringEngine`] essentially defines a runtime for executing Turing Machine drivers
/// according to the specifications defined in a given program. The engine is responsible
/// for managing the execution flow, including tracking the current state, tape position,
/// and handling transitions based on the rules defined in the program.
pub struct TuringEngine<'a, D, Q, A>
where
    D: RawDriver<Q, A>,
    Q: RawState,
{
    /// the actor that will be executing the program
    pub(crate) driver: &'a mut D,
    /// the program being executed
    pub(crate) program: Option<Program<Q, A>>,
    /// the number of cycles executed; independent of the position of the head on the tape
    pub(crate) cycles: usize,
    /// the output tape captures the results of the execution
    pub(crate) output: Vec<A>,
}

impl<'a, D, Q, A> TuringEngine<'a, D, Q, A>
where
    D: RawDriver<Q, A>,
    Q: RawState,
{
    /// initialize a new instance of the engine from the given driver
    pub const fn new(driver: &'a mut D, program: Program<Q, A>) -> Self {
        Self {
            driver,
            output: Vec::new(),
            program: Some(program),
            cycles: 0,
        }
    }    
    /// initialize a new instance of the engine from the given driver
    pub const fn from_driver(driver: &'a mut D) -> Self {
        Self {
            driver,
            output: Vec::new(),
            program: None,
            cycles: 0,
        }
    }
    /// update the current program for the engine
    pub fn set_program(&mut self, program: Program<Q, A>) {
        self.program = Some(program);
    }
    #[inline]
    /// consumes the engine to create another with the given driver
    pub fn with_driver<'b, D2>(self, driver: &'b mut D2) -> TuringEngine<'b, D2, Q, A>
    where
        D2: RawDriver<Q, A>,
    {
        TuringEngine {
            driver,
            program: self.program,
            cycles: self.cycles,
            output: self.output,
        }
    }
    #[inline]
    /// consumes the current instance to create another with the given program
    pub fn with_program(self, program: Program<Q, A>) -> Self {
        TuringEngine {
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
        self.driver
    }
    /// returns a mutable reference to the actor
    pub const fn driver_mut(&mut self) -> &mut D {
        self.driver
    }
    /// returns a reference to the output tape
    pub const fn output(&self) -> &Vec<A> {
        &self.output
    }
    /// returns a mutable reference to the output tape
    pub const fn output_mut(&mut self) -> &mut Vec<A> {
        &mut self.output
    }
    /// returns a reference to the program
    pub const fn program(&self) -> Option<&Program<Q, A>> {
        self.program.as_ref()
    }
    /// returns a mutable reference to the program
    pub const fn program_mut(&mut self) -> Option<&mut Program<Q, A>> {
        self.program.as_mut()
    }
    /// returns a mutable reference to the current steps
    pub fn current_state(&self) -> State<&Q> {
        self.driver().current_state()
    }
    /// returns true if the engine has a program loaded
    pub const fn has_program(&self) -> bool {
        self.program.is_some()
    }
    /// extend the output tape with values from the given iterator
    pub fn extend_output<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = A>,
    {
        self.output_mut().extend(iter);
    }
    /// increments the current epoch by a single unit indicating the end of a cycle or step
    pub const fn next_cycle(&mut self) {
        self.cycles += 1;
    }
    /// reset the engine by clearing the output tape, cycles, and program from the current 
    /// instance
    pub fn reset(&mut self) {
        self.output.clear();
        self.cycles = 0;
        self.program = None;
    }
}
