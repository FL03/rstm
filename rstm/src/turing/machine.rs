/*
    Appellation: tm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Registry;
use crate::prelude::{FsmError, Tape};
use crate::state::{State, Transition};

pub struct Context<T = String> {
    pub(crate) initial_state: State<T>,
    pub(crate) states: Vec<State<T>>,
    pub(crate) transitions: Vec<Transition<T>>,
}

impl<T> Context<T> {
    pub fn new(initial_state: State<T>) -> Self {
        Self {
            initial_state,
            states: Vec::new(),
            transitions: Vec::new(),
        }
    }

    pub fn with_states(self, states: Vec<State<T>>) -> Self {
        Self { states, ..self }
    }

    pub fn with_transitions(self, transitions: Vec<Transition<T>>) -> Self {
        Self {
            transitions,
            ..self
        }
    }

    pub fn initial_state(&self) -> &State<T> {
        &self.initial_state
    }

    pub fn states(&self) -> &Vec<State<T>> {
        &self.states
    }

    pub fn transitions(&self) -> &Vec<Transition<T>> {
        &self.transitions
    }
}

///
pub struct TuringMachine {
    pub(crate) ctx: Context,
    pub(crate) cstate: State,
    pub(crate) registry: Registry,
    pub(crate) tape: Tape,
}

impl TuringMachine {
    pub fn new(initial_state: State, tape: Tape, transforms: Vec<Transition>) -> Self {
        let ctx = Context::new(initial_state.clone());
        let mut transitions = Registry::new();
        for trs in transforms {
            transitions.insert(
                (trs.current_state.clone(), trs.read_symbol),
                (trs.next_state, trs.write_symbol, trs.direction),
            );
        }

        TuringMachine {
            cstate: initial_state,
            ctx,
            tape,
            registry: transitions,
        }
    }

    pub const fn context(&self) -> &Context {
        &self.ctx
    }

    pub const fn current_state(&self) -> &State {
        &self.cstate
    }

    pub const fn tape(&self) -> &Tape {
        &self.tape
    }

    pub fn step(&mut self) -> Result<(), FsmError> {
        let cursor = self.cstate.clone();
        let current_symbol = *self.tape.read()?;
        if let Some(&(ref next_state, write_symbol, direction)) =
            self.registry.get(&(cursor.clone(), current_symbol))
        {
            self.tape.write(write_symbol);
            self.tape.step(direction);
            self.cstate = next_state.clone();
            return Ok(());
        }
        Err(FsmError::state_not_found(cursor, current_symbol))
    }

    pub fn run(&mut self) {
        loop {
            self.tape.print_tape();
            self.step().expect("Error stepping through machine");
            if *self.cstate == "HALT" {
                break;
            }
        }
    }
}
