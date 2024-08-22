/*
    Appellation: exec <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Actor;
use crate::{Error, Head, Program, Symbolic};

pub struct Executor<Q, S> {
    pub(crate) actor: Actor<Q, S>,
    pub(crate) program: Program<Q, S>,
    /// the number of steps taken by the actor
    pub(crate) steps: usize,
}

impl<Q, S> Executor<Q, S> {
    pub(crate) fn new(actor: Actor<Q, S>, program: Program<Q, S>) -> Self {
        Self {
            actor,
            program,
            steps: 0,
        }
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
            steps: 0,
        }
    }
    /// Load a program into the executor
    pub fn load(self, program: Program<Q, S>) -> Self {
        Executor { program, ..self }
    }

    pub const fn actor(&self) -> &Actor<Q, S> {
        &self.actor
    }
    /// Reads the current symbol at the head of the tape
    pub fn read(&self) -> Result<Head<&Q, &S>, Error> {
        self.actor.read()
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "run", target = "actor")
    )]
    pub fn run(&mut self) -> Result<(), Error>
    where
        Q: Clone + PartialEq + 'static,
        S: Symbolic,
    {
        #[cfg(feature = "tracing")]
        tracing::info!("Running the program...");
        for _h in self {
            continue;
        }
        Ok(())
    }
}

impl<Q, S> Iterator for Executor<Q, S>
where
    Q: Clone + PartialEq + 'static,
    S: Symbolic,
{
    type Item = Head<Q, S>;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "next", target = "actor")
    )]
    fn next(&mut self) -> Option<Self::Item> {
        // increment the number of steps taken
        self.steps += 1;
        #[cfg(feature = "tracing")]
        tracing::info!("{tape:?}", tape = self.actor());
        // check if the actor is halted
        if self.actor.is_halted() {
            #[cfg(feature = "tracing")]
            tracing::warn!("Detected a halted state; terminating the program...");
            return None;
        }
        // read the tape
        let head = if let Ok(cur) = self.read() {
            cur.cloned()
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!("Unable to locate the value of the head...");
            Head::from_state(self.actor.state().cloned())
        };
        // execute the program
        if let Some(tail) = self.program.get(&head).cloned() {
            // process the instruction
            self.actor.process(tail.clone());
            // return the head
            return Some(tail.into_head());
        } else {
            #[cfg(feature = "tracing")]
            tracing::error!("No symbol found at {}", self.actor.position());
            panic!("No symbol found at {}", self.actor.position());
        }
    }
}
