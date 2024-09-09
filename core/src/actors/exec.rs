/*
    Appellation: exec <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Actor;
use crate::{Error, Head, Ruleset, State, Symbolic};

/// # [Executor]
/// 
/// The [Executor] struct is directly responsible for the execution of a program. From a rustic
/// perspective, the [Executor] is an iterator that reads the current symbol at the head of 
/// the tape,
pub struct Executor<Q, S> {
    /// the actor that will be executing the program
    pub(crate) actor: Actor<Q, S>,
    /// the program being executed
    pub(crate) program: Ruleset<Q, S>,
    /// the number of steps taken by the actor
    pub(crate) steps: usize,
}

impl<Q, S> Executor<Q, S> {
    pub(crate) fn new(actor: Actor<Q, S>, program: Ruleset<Q, S>) -> Self {
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
            program: Ruleset {
                initial_state: Default::default(),
                rules: Vec::new(),
            },
            steps: 0,
        }
    }
    /// Load a program into the executor
    pub fn load(self, program: Ruleset<Q, S>) -> Self {
        Executor { program, ..self }
    }

    pub const fn actor(&self) -> &Actor<Q, S> {
        &self.actor
    }

    pub fn current_state(&self) -> State<&'_ Q> {
        self.actor.state()
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
            cur
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!("[Index Error] the current position ({pos}) of the head is out of bounds, assuming the symbol to be its default value...", pos = self.actor.head.symbol);
            Head {
                state: self.actor.state(),
                symbol: &S::default(),
            }
        };
        // execute the program
        if let Some(tail) = self.program.get(head.state, head.symbol) {
            let next = tail.as_head().cloned();
            // process the instruction
            let _prev = self
                .actor
                .step(tail.direction, tail.state.clone(), tail.symbol);
            // return the head
            Some(next)
        } else {
            #[cfg(feature = "tracing")]
            tracing::error!("No symbol found at {}", self.actor.position());
            panic!("No symbol found at {}", self.actor.position());
        }
    }
}
