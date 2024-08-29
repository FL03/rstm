/*
    Appellation: instructions <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Head, State, Tail};
use std::collections::HashMap;


#[derive(Default)]
pub struct RulesetBuilder<Q, S> {
    pub(crate) initial_state: Option<State<Q>>,
    pub(crate) rules: HashMap<Head<Q, S>, Tail<Q, S>>,
}