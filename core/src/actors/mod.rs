/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::actor::Actor;

pub(crate) mod actor;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::actor::Actor;
}

use crate::{Program, Symbolic};

pub struct Model<Q, S> {
    pub actor: Actor<Q, S>,
    pub program: Program<Q, S>,
}

impl<Q, S> Model<Q, S> {
    pub fn with_program(self, program: Program<Q, S>) -> Self {
        Model { program, ..self }
    }
}
impl<Q, S> Iterator for Model<Q, S>
where
    Q: Clone + PartialEq,
    S: Symbolic,
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.actor.is_halted() {
            return None;
        }
        let state = self.actor.state();
        let symbol = self.actor.read();
        let rule = self.program.get(state, symbol)?;
        self.actor.handle(rule.clone());
        unimplemented!()
    }
}
