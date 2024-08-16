/*
    Appellation: exec <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Actor;
use crate::{Error, Head, Program, Symbolic};

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

    pub fn with_program(self, program: Program<Q, S>) -> Self {
        Executor { program, ..self }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "run", target = "actor")
    )]
    pub fn run(&mut self) -> Result<Vec<S>, Error>
    where
        Q: Clone + PartialEq + core::fmt::Debug + 'static,
        S: Symbolic,
    {
        #[cfg(feature = "tracing")]
        tracing::info!("Executing the program using the given actor...");
        loop {
            match self.next() {
                Some(_) => continue,
                None => break,
            }
        }
        Ok(self.actor.alpha().to_vec())
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
        }
        match self.actor.read() {
            Some(Head { state, symbol }) => {
                #[cfg(feature = "tracing")]
                tracing::info!("{tape:?}", tape = self.actor);
                let rule = self.program.get(state, symbol).expect("No instruction found for the current head");
                return self.actor.step(rule).map(|h| h.cloned());
            }
            None => {
                #[cfg(feature = "tracing")]
                tracing::error!("No instruction found for the current head");
                panic!("No head found at the current position");
            }
        }
        
    }
}
