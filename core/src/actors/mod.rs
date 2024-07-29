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

use crate::Program;

pub struct Model<Q, S> {
    pub actor: Actor<Q, S>,
    pub input: Vec<S>,
    pub program: Program,
}

impl<Q, S> Iterator for Model<Q, S>
where
    Q: Clone,
    S: Clone,
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        if self.actor.is_halted() {
            return None;
        }

        let state = self.actor.head.state.clone();
        let symbol = self.actor.read().clone();
        Some(symbol)
    }
}
