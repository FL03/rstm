/*
    Appellation: engine <module>
    Created At: 2025.08.31:14:49:50
    Contrib: @FL03
*/
mod impl_turing_engine;

#[allow(deprecated)]
mod impl_deprecated;

use crate::tmh::TMH;
use alloc::vec::Vec;
use rstm_programs::Program;
use rstm_state::{RawState, State};

/// The [`TuringEngine`] implementation is essentially a runtime for Turing machine, allowing
pub struct TuringEngine<'a, Q, A>
where
    Q: RawState,
{
    /// the actor that will be executing the program
    pub(crate) driver: &'a mut TMH<Q, A>,
    /// the program being executed
    pub(crate) program: Option<Program<Q, A>>,
    /// the number of cycles executed; independent of the position of the head on the tape
    pub(crate) cycles: usize,
    pub(crate) output: Vec<A>,
}

impl<'a, Q, A> TuringEngine<'a, Q, A>
where
    Q: RawState,
{
    pub const fn new(driver: &'a mut TMH<Q, A>) -> Self {
        Self {
            driver,
            output: Vec::new(),
            program: None,
            cycles: 0,
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
    pub const fn output(&self) -> &Vec<A> {
        &self.output
    }
    /// returns a reference to the program
    pub fn program(&self) -> crate::Result<&Program<Q, A>> {
        self.program.as_ref().ok_or(crate::Error::NoProgram)
    }
    /// returns a mutable reference to the program
    pub fn program_mut(&mut self) -> crate::Result<&mut Program<Q, A>> {
        self.program.as_mut().ok_or(crate::Error::NoProgram)
    }
    /// returns a copy of the total number of cycles, or steps, the engine has preformed
    pub const fn cycles(&self) -> usize {
        self.cycles
    }
    /// returns a mutable reference to the current steps
    pub const fn current_state(&self) -> &State<Q> {
        self.driver().state()
    }
    /// returns true if the engine has a program loaded
    pub const fn has_program(&self) -> bool {
        self.program.is_some()
    }
}
