/*
    Appellation: engine <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::RawState;
use crate::{Error, Head, Program};

use std::collections::HashMap;

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
    pub fn read(&self) -> Option<&A> {
        self.tape.get(&self.scope.symbol)
    }

    pub fn write(&mut self, symbol: A) {
        self.tape.insert(self.scope.symbol, symbol);
    }

    pub fn process<I>(&mut self) -> Result<(), Error>
    where
        A: crate::Symbolic,
        Q: Clone + Eq + std::hash::Hash,
    {
        let symbol = self.read().expect("no symbol found");
        if let Some(rule) = self.program.get(self.scope.state(), symbol) {
            let next = Head {
                state: rule.state.clone(),
                symbol: self.scope.symbol + rule.direction,
            };
            self.write(rule.symbol);
            self.scope = next;
        }

        Ok(())
    }
}
