/*
    Appellation: transition <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Direction, Head};

pub struct Shift<'a, Q, S> {
    pub direction: Direction,
    pub head: &'a Head<Q, S>,
}
