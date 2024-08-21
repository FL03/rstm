/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{halt::Halt, state::State, states::*};

pub(crate) mod state;

pub mod halt;

pub(crate) mod states {
    #[doc(inline)]
    pub use self::binary::*;

    pub(crate) mod binary;
}

pub(crate) mod prelude {

    pub use super::state::State;
    pub use super::states::*;
}

pub type AnyState = State<Box<dyn std::any::Any + Send + Sync>>;

#[doc(hidden)]
pub trait RawState {
    type Ctx;
}

pub trait Stated<Q>: RawState<Ctx = Q> {
    fn cloned(&self) -> Self
    where
        Q: Clone;
    fn copied(&self) -> Self
    where
        Q: Copy;
}

#[doc(hidden)]
pub trait Stateful<Q> {
    type State: RawState<Ctx = Q>;
}

/*
 ************* Implementations *************
*/
impl<Q> RawState for Halt<Q> {
    type Ctx = Q;
}

impl<Q> RawState for State<Q> {
    type Ctx = Q;
}

impl<Q> Stateful<Q> for State<Q> {
    type State = State<Q>;
}
