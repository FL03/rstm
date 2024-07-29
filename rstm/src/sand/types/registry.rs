/*
    Appellation: registry <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::rules::Instruction;
use std::collections::hash_map::{self, HashMap};

type Store<Q, S> = HashMap<usize, Instruction<Q, S>>;

pub struct Registry<Q = String, S = char> {
    pub(crate) rules: HashMap<isize, Instruction<Q, S>>,
}

pub struct Actor<Q, S> {
    pub(crate) pos: isize, //
    pub(crate) rule: Instruction<Q, S>,
}
