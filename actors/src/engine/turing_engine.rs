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
use rstm_rules::Program;
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
    /// the current epoch, or step, of the machine
    pub(crate) epoch: usize,
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
}