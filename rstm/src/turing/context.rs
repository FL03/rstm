/*
    Appellation: context <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::rules::Program;
use crate::state::State;

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Context<Q = String, S = char> {
    pub(crate) program: Program<Q, S>,
    pub(crate) state: Option<State<Q>>,
}

impl<Q, S> Context<Q, S> {
    pub fn new(program: Program<Q, S>, state: State<Q>) -> Self {
        Self {
            program,
            state: Some(state),
        }
    }
    ///
    pub fn from_program(program: Program<Q, S>) -> Self {
        Self { program, state: None }
    }

    pub fn from_state(state: State<Q>) -> Self where Q: Default, S: Default {
        Self {
            program: Program::default(),
            state: Some(state),
        }
    }

    
    /// Returns the current state of the system;
    /// if the state is [none](Option::None), assumes the initial state.
    pub fn current_state(&self) -> &State<Q> {
        self.state.as_ref().unwrap_or(self.initial_state())
    }

    pub const fn initial_state(&self) -> &State<Q> {
        &self.program.initial_state()
    }

    pub fn program(&self) -> &Program<Q, S> {
        &self.program
    }

    pub fn program_mut(&mut self) -> &mut Program<Q, S> {
        &mut self.program
    }

    pub fn set_state(&mut self, state: State<Q>) {
        self.state = Some(state);
    }
}
