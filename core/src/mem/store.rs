/*
    Appellation: container <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Rule, State};

#[doc(hidden)]
pub trait GetRule<Q, S> {
    fn get(&self, state: State<&Q>, symbol: &S) -> Option<&Rule<Q, S>>;

    fn get_mut(&mut self, state: State<&Q>, symbol: &S) -> Option<&mut Rule<Q, S>>;
}

pub trait RawContainer {
    type Elem;
}

/*
 ************* Implementations *************
*/

impl<T> RawContainer for Vec<T> {
    type Elem = T;
}

impl<T> RawContainer for Box<[T]> {
    type Elem = T;
}

impl<T> RawContainer for [T] {
    type Elem = T;
}

impl<T> RawContainer for &mut [T] {
    type Elem = T;
}

impl<const N: usize, T> RawContainer for &mut [T; N] {
    type Elem = T;
}
