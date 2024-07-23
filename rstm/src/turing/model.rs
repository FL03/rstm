/*
    Appellation: tm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Context;

use crate::prelude::{FsmError, Head, Symbolic, Tape};
use crate::rules::{Instruction, Program};
use crate::state::{Haltable, State};

/// # Turing Machine ([TM])
///
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TM<Q = String, S = char> {
    pub(crate) ctx: Context<Q, S>,
    pub(crate) tape: Tape<S>,
}

impl<Q, S> TM<Q, S> {
    pub fn new(
        initial_state: State<Q>,
        instructions: impl IntoIterator<Item = Instruction<Q, S>>,
        tape: Tape<S>,
    ) -> Self
    where
        Q: Clone,
        S: Clone + Default,
    {
        let program = Program::new(initial_state.clone()).with_instructions(instructions);
        let ctx = Context::new(program, initial_state);
        TM { ctx, tape }
    }

    pub const fn context(&self) -> &Context<Q, S> {
        &self.ctx
    }

    pub fn current_state(&self) -> &State<Q> {
        self.context().current_state()
    }

    pub fn head(&self) -> Head<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        let state = self.current_state().clone();
        let symbol = self.tape.read().unwrap().clone();
        Head::new(state, symbol)
    }

    pub const fn tape(&self) -> &Tape<S> {
        &self.tape
    }

    pub fn tape_mut(&mut self) -> &mut Tape<S> {
        &mut self.tape
    }
}

// #[cfg(feature = "std")]
impl<Q, S> TM<Q, S>
where
    Q: Clone + PartialEq,
    S: Clone + Symbolic,
{
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "step", target = "fsm")
    )]
    pub fn step(&mut self) -> Result<(), FsmError> {
        #[cfg(feature = "tracing")]
        tracing::info!("Stepping...");
        let prog = self.ctx.program.clone();
        // Get a clone of the current state
        let cst = self.current_state().clone();
        let sym = self.tape().read()?.clone();
        let head = Head::new(cst.clone(), sym);
        if let Some(tail) = prog.get_head(&head).first().cloned() {
            let nxt = self.tape.update_inplace(tail.clone());
            self.ctx.set_state(nxt);
            return Ok(());
        }
        Err(FsmError::state_not_found(""))
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "run", target = "fsm")
    )]
    pub fn run(mut self) -> Result<(), FsmError> {
        #[cfg(feature = "tracing")]
        tracing::info!("Running the program...");
        loop {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", &self.tape);
            match self.step() {
                Ok(_) => continue,
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}

impl<Q> TM<Q>
where
    Q: Clone + Eq + core::hash::Hash + Haltable,
{
    pub fn run_haltable(&mut self) -> Result<(), FsmError> {
        let _ = loop {
            dbg!(self.tape());
            match self.step() {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
            }

            if self.current_state().halt() {
                break;
            }
        };

        Ok(())
    }
}
