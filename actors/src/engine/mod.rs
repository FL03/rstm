/*
    Appellation: engine <module>
    Created At: 2025.09.03:21:12:32
    Contrib: @FL03
*/
//! The [`engine`](self) module provides core 

#[cfg(feature = "alloc")]
#[doc(inline)]
pub use self::turing_engine::TuringEngine;

#[cfg(feature = "alloc")]
pub mod turing_engine;

pub(crate) mod prelude {
    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::turing_engine::*;
    #[doc(inline)]
    pub use super::{RawEngine, Engine};
}


use crate::traits::RawActor;

use rstm_rules::Program;
use rstm_state::RawState;

/// The [`RawEngine`] trait defines the basis for compatible engines within the system.
pub trait RawEngine<Q, A>
where
    Q: RawState,
{
    type Driver: RawActor;

    private!();
}

/// An [`Engine`] is responsible for processing input according to some set of pre-defined
/// rules using the configured driver.
pub trait Engine<Q, S>: RawEngine<Q, S>
where
    Q: RawState,
{
    fn load(&mut self, program: Program<Q, S>);

    fn run(&mut self) -> crate::Result<()>;
}