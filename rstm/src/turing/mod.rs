/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::machine::TuringMachine;

pub(crate) mod machine;

#[doc(hidden)]
pub mod context;
#[doc(hidden)]
pub mod utm;

pub(crate) mod prelude {
    pub use super::machine::TuringMachine;
}

use crate::prelude::{Direction, State};
use std::collections::HashMap;
///
pub(crate) type Registry<Q = String> = HashMap<(State<Q>, char), (State<Q>, char, Direction)>;

pub trait Turing {}

pub trait Alphabet {
    fn symbols(&self) -> Vec<char>;
}
