/*
    Appellation: builder <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::rules::{Program, Rule};
use crate::State;
pub struct ProgramBuilder<Q, S> {
    initial_state: Option<State<Q>>,
    rules: Vec<Rule<Q, S>>,
}

impl<Q, S> ProgramBuilder<Q, S> {
    pub fn new() -> Self {
        Self {
            initial_state: None,
            rules: Vec::new(),
        }
    }

    pub fn initial_state(self, State(state): State<Q>) -> Self {
        Self {
            initial_state: Some(State(state)),
            ..self
        }
    }

    pub fn rules<I>(self, rules: I) -> Self
    where
        I: IntoIterator<Item = Rule<Q, S>>,
    {
        Self {
            rules: Vec::from_iter(rules),
            ..self
        }
    }

    pub fn build(self) -> Program<Q, S> {
        Program {
            initial_state: self.initial_state.unwrap(),
            rules: self.rules,
        }
    }
}
