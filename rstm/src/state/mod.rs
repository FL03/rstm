/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{state::State, transition::Transition};

pub(crate) mod state;
pub(crate) mod transition;

pub(crate) mod prelude {
    pub use crate::state::{State, Transition};
}


#[allow(unused)]
#[doc(hidden)]
mod private {
    pub trait State {
        type Data;
    }

    /// [Stateful] is used to describe objects which rely upon a state.
    ///
    pub trait Stateful<T = String> {
        type State: State<Data = T>;
    }

    pub enum TimePerspective<T> {
        Past(T),
        Present(T),
        Future(T),
    }

    pub enum StateGroup<S> where S: State {
        A {
            prev: S,
            curr: S,
        },
        B {
            curr: S,
            next: S,
        },
        C {
            prev: S,
            next: S,
        },
        D {
            prev: S,
            curr: S,
            next: S,
        }
    }



}