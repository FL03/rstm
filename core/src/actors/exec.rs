/*
    Appellation: exec <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Actor;
use crate::{Head, Program, Symbolic};

pub struct Executor<Q, S> {
    pub(crate) actor: Actor<Q, S>,
    pub(crate) program: Program<Q, S>,
}

impl<Q, S> Executor<Q, S> {
    pub fn from_actor(actor: Actor<Q, S>) -> Self
    where
        Q: Default,
    {
        Self {
            actor,
            program: Program::new(),
        }
    }

    pub fn with_program(self, program: Program<Q, S>) -> Self {
        Executor { program, ..self }
    }
}

impl<Q, S> Iterator for Executor<Q, S>
where
    Q: Clone + PartialEq + 'static,
    S: Symbolic,
{
    type Item = Head<Q, S>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.actor.is_halted() {
            return None;
        }
        let state = self.actor.state();
        let symbol = self.actor.read();
        let rule = self.program.get(state, symbol)?;
        Some(self.actor.handle(rule).cloned())
    }
}
