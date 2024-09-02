/*
    Appellation: cell <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Alphabet, Head, State};

pub struct Cell<Q, S>
where
    S: Alphabet,
{
    pub index: isize,
    pub state: State<*const Q>,
    pub symbol: *const S::Elem,
}

pub struct Snapshot<Q, S>
where
    S: Alphabet,
{
    pub index: isize,
    pub head: Head<Q, *const S::Elem>,
    pub tape: S,
}
