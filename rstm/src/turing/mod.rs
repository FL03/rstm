/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{actor::Actor, context::Context, machine::Fsm};

pub(crate) mod actor;
pub(crate) mod context;
pub(crate) mod machine;

pub(crate) mod prelude {
    pub use super::machine::Fsm;
}

#[doc(hidden)]
pub trait Turing {}
