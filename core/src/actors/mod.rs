/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{actor::Actor, exec::Executor};

pub(crate) mod actor;
pub(crate) mod exec;

pub(crate) mod prelude {
    pub use super::actor::Actor;
    pub use super::exec::Executor;
}

use crate::{rules::Program, Alphabet};

pub trait Model {
    type Alpha: Alphabet;
}
pub trait Runtime<Q, S> {
    fn load(&mut self, program: Program<Q, S>);

    fn run(&mut self);
}
