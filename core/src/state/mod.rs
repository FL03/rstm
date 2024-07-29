/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{state::State, states::*};

pub(crate) mod state;

pub(crate) mod states {
    #[doc(inline)]
    pub use self::{binary::*, halting::*};

    pub(crate) mod binary;
    pub(crate) mod halting;
}

pub(crate) mod prelude {

    pub use super::state::State;
    pub use super::states::*;
}

#[doc(hidden)]
pub trait RawState {
    type Data;
}

#[doc(hidden)]
pub trait Stateful<Q> {
    type State: RawState<Data = Q>;
}

/*
 ************* Implementations *************
*/
impl<Q> RawState for State<Q> {
    type Data = Q;
}

impl<Q> Stateful<Q> for State<Q> {
    type State = State<Q>;
}
