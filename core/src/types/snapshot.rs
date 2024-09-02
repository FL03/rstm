/*
    Appellation: snapshot <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Alphabet, Head};

pub struct Snapshot<Q, S>
where
    S: Alphabet,
{
    pub index: isize,
    pub head: Head<Q, *const S::Elem>,
    pub tape: S,
}
