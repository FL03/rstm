/*
    Appellation: engine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(feature = "std")]
use crate::state::RawState;
use crate::{Direction, Error, Head, Program, State};

use std::collections::hash_map::{self, HashMap};

pub struct Engine<Q, A>
where
    Q: RawState,
{
    pub(crate) program: Program<Q, A>,
    pub(crate) scope: Head<Q, isize>, // Head<Q, *const A::Elem>,
    pub(crate) tape: HashMap<isize, A>,
}

impl<Q, A> Engine<Q, A>
where
    Q: RawState,
{
    // pub fn new(initial_state: State<Q>) -> Self {
    //     Self {
    //         program: Program::new().initial_state(state),
    //         scope: Head {
    //             state: initial_state,
    //             symbol: 0,
    //         },
    //         tape: HashMap::new(),
    //     }
    // }
    pub fn with_input<I>(self, input: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        Self {
            tape: HashMap::from_iter(input.into_iter().enumerate().map(|(i, a)| (i as isize, a))),
            ..self
        }
    }
    pub fn with_program(self, program: Program<Q, A>) -> Self {
        Self { program, ..self }
    }

    pub fn position(&self) -> isize {
        self.scope.symbol
    }

    pub fn cell(&self) -> &Head<Q, isize> {
        &self.scope
    }

    pub fn entry(&mut self) -> hash_map::Entry<'_, isize, A> {
        self.tape.entry(self.position())
    }

    pub fn read(&self) -> Option<&A> {
        self.tape.get(&self.scope.symbol)
    }

    pub fn write(&mut self, data: A) -> Option<A> {
        self.tape.insert(self.position(), data)
    }

    fn handle(&mut self, direction: Direction, state: State<Q>, symbol: A) {
        let next = Head {
            state,
            symbol: self.position() + direction,
        };
        core::mem::replace(&mut self.scope, next);
    }

    pub fn process(&mut self) -> Result<(), Error>
    where
        A: crate::Symbolic,
        Q: Clone + Eq + std::hash::Hash,
    {
        let scope = self.cell();
        let symbol = match self.read() {
            Some(symbol) => symbol,
            None => return Err(Error::runtime_error("Engine::process")),
        };

        if let Some(rule) = self.program.get(self.scope.state(), symbol) {
            self.handle(rule.direction, rule.state.clone(), rule.symbol);
        }

        Ok(())
    }
}
