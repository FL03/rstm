/*
    Appellation: engine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(feature = "std")]
use crate::rules::Rule;
use crate::state::RawState;
use crate::{Direction, Error, Head, Ruleset, State};

use std::collections::hash_map::{self, HashMap};

pub struct TMH<Q, A = char>
where
    Q: RawState,
{
    pub(crate) program: Ruleset<Q, A>,
    pub(crate) scope: Head<Q, isize>, // Head<Q, *const A::Elem>,
    pub(crate) tape: HashMap<isize, A>,
}

impl<Q, A> TMH<Q, A>
where
    Q: RawState,
{
    pub fn new(initial_state: State<Q>) -> Self {
        Self {
            program: Ruleset::new(),
            scope: Head {
                state: initial_state,
                symbol: 0,
            },
            tape: HashMap::new(),
        }
    }
    pub fn with_input<I>(self, input: I) -> Self
    where
        I: IntoIterator<Item = A>,
    {
        Self {
            tape: HashMap::from_iter(input.into_iter().enumerate().map(|(i, a)| (i as isize, a))),
            ..self
        }
    }
    pub fn with_program(self, program: Ruleset<Q, A>) -> Self {
        Self { program, ..self }
    }

    pub fn load<I>(&mut self, rules: I)
    where
        I: IntoIterator<Item = Rule<Q, A>>,
    {
        self.program.extend(rules);
    }

    pub fn current_state(&self) -> State<&'_ Q> {
        self.scope.state()
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

    pub fn read(&self) -> Option<Head<&'_ Q, &'_ A>> {
        self.tape.get(&self.scope.symbol).map(|symbol| Head {
            state: self.scope.state(),
            symbol,
        })
    }

    pub fn write(&mut self, data: A) -> Option<A> {
        self.tape.insert(self.position(), data)
    }
    /// Handle the given rule; writes the symbol onto the tape before updating the state of the
    /// head and applying a single step in the direction specified. Returns the previous head.
    fn handle(&mut self, direction: Direction, state: State<Q>, symbol: A) -> Head<Q, isize> {
        // write the symbol onto the tape
        let _prev = self.write(symbol);
        // replace the current head with the new head
        self.scope.replace(state, self.position() + direction)
    }

    pub fn process(&mut self) -> Result<(), Error>
    where
        A: crate::Symbolic,
        Q: Clone + Eq + std::hash::Hash,
    {
        let Head { state, symbol } = match self.read() {
            Some(head) => head,
            None => return Err(Error::runtime_error("Engine::process")),
        };

        if let Some(rule) = self.program.get(state, symbol) {
            let _prev = self.handle(rule.direction, rule.state.clone(), rule.symbol);
        }

        Ok(())
    }
}
