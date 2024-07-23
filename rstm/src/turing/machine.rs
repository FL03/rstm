/*
    Appellation: tm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Context;

use crate::prelude::{FsmError, Head, Registry, Tape};
use crate::rules::Instruction;
use crate::state::{Haltable, State};
use crate::Symbolic;

/// # Finite State Machine
/// 

#[derive(Clone, Debug,)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Fsm<Q = String, S = char> {
    pub(crate) ctx: Context<Q, S>,
    pub(crate) registry: Registry<Q, S>,
    pub(crate) tape: Tape<S>,
}

impl<Q, S> Fsm<Q, S> {
    pub const fn context(&self) -> &Context<Q, S> {
        &self.ctx
    }

    pub fn current_state(&self) -> &State<Q> {
        self.context().current_state()
    }

    pub fn head(&self) -> Head<Q, S> where Q: Clone, S: Clone {
        let state = self.current_state().clone();
        let symbol = self.tape.read().unwrap().clone();
        Head::new(state, symbol)
    }
    ///
    pub const fn registry(&self) -> &Registry<Q, S> {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut Registry<Q, S> {
        &mut self.registry
    }

    pub const fn tape(&self) -> &Tape<S> {
        &self.tape
    }

    pub fn tape_mut(&mut self) -> &mut Tape<S> {
        &mut self.tape
    }
}


#[cfg(feature = "std")]
impl<Q, S> Fsm<Q, S>
where
    Q: Eq + core::hash::Hash,
    S: Symbolic
{
    pub fn new(
        initial_state: State<Q>,
        instructions: impl IntoIterator<Item = Instruction<Q, S>>,
        tape: Tape<S>,
        
    ) -> Self where Q: Clone + Default, S: Clone + Default {
        let ctx = Context::from_state(initial_state.clone());
        let mut registry = Registry::new();
        for t in instructions {
            registry.insert(t.head, t.tail);
        }

        Fsm {
            ctx,
            tape,
            registry,
        }
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, name = "step", target = "fsm"))]
    pub fn step(&mut self) -> Result<(), FsmError> where Q: Clone, S: Clone {
        #[cfg(feature = "tracing")]
        tracing::info!("Stepping...");
        let registry = self.registry.clone();
        // Get a clone of the current state
        let cst = self.current_state().clone();
        let sym = self.tape().read()?.clone();
        let head = Head::new(cst.clone(), sym);
        if let Some(tail) = registry.get(&head).cloned() {
            let nxt = self.tape.update(tail);
            self.ctx.set_state(nxt);
            return Ok(());
        }
        Err(FsmError::state_not_found(""))
    }
    #[cfg_attr(feature = "tracing", tracing::instrument(skip_all, name = "run", target = "fsm"))]
    pub fn run(mut self) -> Result<(), FsmError> where Q: Clone, S: Clone {
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

impl<Q> Fsm<Q>
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
