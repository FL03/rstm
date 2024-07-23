/*
    Appellation: stateful <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// S
pub trait BaseState {
    type Data;

    fn data(&self) -> &Self::Data;

    fn data_mut(&mut self) -> &mut Self::Data;

    #[doc(hidden)]
    fn swap<S>(&mut self, state: S::Data) -> S
    where
        S: BaseState;
}

/// [Stateful] is used to describe objects which rely upon a state.
///
pub trait Stateful<T = String> {
    type State: BaseState<Data = T>;
}

#[allow(unused)]
#[doc(hidden)]
mod private {
    use super::BaseState;

    pub enum TimePerspective<T> {
        Past(T),
        Present(T),
        Future(T),
    }

    pub enum StateGroup<S>
    where
        S: BaseState,
    {
        A { prev: S, curr: S },
        B { curr: S, next: S },
        C { prev: S, next: S },
        D { prev: S, curr: S, next: S },
    }
}
