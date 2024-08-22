/*
    Appellation: exec <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Actor;
use crate::{Error, Head, Program, Symbolic, Tail};

pub struct Executor<Q, S> {
    pub(crate) actor: Actor<Q, S>,
    pub(crate) program: Program<Q, S>,
}

impl<Q, S> Executor<Q, S> {
    pub(crate) fn new(actor: Actor<Q, S>, program: Program<Q, S>) -> Self {
        Self { actor, program }
    }

    pub fn from_actor(actor: Actor<Q, S>) -> Self
    where
        Q: Default,
    {
        Self {
            actor,
            program: Program {
                initial_state: Default::default(),
                rules: Vec::new(),
            },
        }
    }
    /// Load a program into the executor
    pub fn load(self, program: Program<Q, S>) -> Self {
        Executor { program, ..self }
    }

    pub fn actor(&self) -> &Actor<Q, S> {
        &self.actor
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "run", target = "actor")
    )]
    pub fn run(&mut self) -> Result<(), Error>
    where
        Q: Clone + PartialEq + core::fmt::Debug + 'static,
        S: Symbolic,
    {
        #[cfg(feature = "tracing")]
        tracing::info!("Running the program...");
        for i in self {
            #[cfg(feature = "tracing")]
            tracing::info!("{head:?}", head = i);
        }
        Ok(())
    }
}

impl<Q, S> Iterator for Executor<Q, S>
where
    Q: Clone + PartialEq + core::fmt::Debug + 'static,
    S: Symbolic,
{
    type Item = Head<Q, S>;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "next", target = "actor")
    )]
    fn next(&mut self) -> Option<Self::Item> {
        if self.actor.is_halted() {
            #[cfg(feature = "tracing")]
            tracing::info!("Halted");
            return None;
        } else if let Some(h) = self.actor().read() {
            #[cfg(feature = "tracing")]
            tracing::info!("{tape:?}", tape = self.actor());
            let Tail {
                direction,
                state,
                symbol,
            } = self
                .program
                .get_head_ref(h)
                .expect("No instruction found for the current head");
            self.actor.handle(direction, state.cloned(), *symbol);
            return self.actor.read().map(|h| h.cloned());
        } else {
            #[cfg(feature = "tracing")]
            tracing::error!("No instruction found for the current head");
            panic!("No head found at the current position");
        }
    }
}
