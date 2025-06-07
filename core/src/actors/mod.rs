/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! This modules implements an [Actor] struct, which is a Turing machine with a moving head
//! (TMH).
#[doc(inline)]
pub use self::{actor::Actor, exec::Executor};

pub(crate) mod actor;
pub(crate) mod exec;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::actor::Actor;
    #[doc(inline)]
    pub use super::exec::Executor;
}

use crate::state::RawState;
use crate::{Error, Rule, Tail};

/// [Handle] describes the step-by-step execution of a program; the trait is generalized
/// with the introduction of a single generic parameter, `T`, capable of sufficiently
/// representing any possible object that may be passed to the [handle] method.
///
/// This notion is particularly useful as it allows us to define the process using an actor,
/// before generically implementing the [Engine] trait for the [Executor] struct. Doing so
/// allows for further abstraction by considering the
pub trait Handle<T> {
    type Output;

    fn handle(&mut self, args: T) -> Self::Output;
}

#[doc(hidden)]
pub trait Engine<Q, S>: Handle<Tail<Q, S>>
where
    Q: RawState,
{
    fn load<I>(&mut self, program: I)
    where
        I: IntoIterator<Item = Rule<Q, S>>;

    fn run(&mut self) -> Result<(), Error>;
}
