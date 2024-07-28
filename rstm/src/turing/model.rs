/*
    Appellation: tm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Context;

use crate::prelude::{Error, Head, StdTape, Symbolic, SymbolicExt};
use crate::rules::{Instruction, Program};
use crate::state::{Haltable, State};

/// # Turing Machine ([TM])
///
/// The Turing Machine is a mathematical model of computation that uses a set of rules to determine
/// how a machine should manipulate a tape. The machine can read, write, and move linearly across the tape.
/// Each pre-defined rule maps a head, consisting of a state and symbol, to a new state and symbol along with a direction.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TM<Q = String, S = char> {
    pub(crate) ctx: Context<Q, S>,
    pub(crate) tape: StdTape<S>,
}

impl<Q, S> TM<Q, S> {
    pub fn new(
        initial_state: State<Q>,
        instructions: impl IntoIterator<Item = Instruction<Q, S>>,
        tape: StdTape<S>,
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

    pub fn context_mut(&mut self) -> &mut Context<Q, S> {
        &mut self.ctx
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

    pub const fn tape(&self) -> &StdTape<S> {
        &self.tape
    }

    pub fn tape_mut(&mut self) -> &mut StdTape<S> {
        &mut self.tape
    }
}

// #[cfg(feature = "std")]
impl<Q, S> TM<Q, S>
where
    Q: Clone + PartialEq,
    S: SymbolicExt,
{
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "step", target = "fsm")
    )]
    pub fn step(&mut self) -> Result<Head<Q, S>, Error> {
        #[cfg(feature = "tracing")]
        tracing::info!("Stepping...");
        let prog = self.ctx.program.clone();
        // Get a clone of the current state
        let cst = self.current_state().clone();
        let sym = self.tape().read()?.clone();
        let head = Head::new(cst.clone(), sym);
        if let Some(&tail) = prog.get_head(&head).first() {
            let head = tail.next_head();
            let nxt = self.tape.update_inplace(tail.clone());
            self.ctx.set_state(nxt.clone());
            return Ok(Head::new(nxt, tail.write_symbol().clone()));
        }
        Err(Error::state_not_found(""))
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "run", target = "fsm")
    )]
    pub fn run(mut self) -> Result<(), Error> {
        #[cfg(feature = "tracing")]
        tracing::info!("Running the program...");
        loop {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", &self.tape);
            match self.step() {
                Ok(_) => {
                    // if self.current_state().is_halt() {
                    //     return Ok(());
                    // }
                    continue;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}
