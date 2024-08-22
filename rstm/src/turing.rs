/*
    Appellation: tm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::{model::Turing, state::TMS};

pub(crate) mod model;
pub(crate) mod state;

use crate::prelude::{Error, Head, Symbolic, Tail, Tape};
use crate::rules::Program;
use crate::state::State;

/// # Turing Machine ([TM])
///
/// The Turing Machine is a mathematical model of computation that uses a set of rules to determine
/// how a machine should manipulate a tape. The machine can read, write, and move linearly across the tape.
/// Each pre-defined rule maps a head, consisting of a state and symbol, to a new state and symbol along with a direction.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TM<Q = String, S = char> {
    pub(crate) program: Program<Q, S>,
    pub(crate) state: State<Q>,
    pub(crate) tape: Tape<S>,
}

impl<Q, S> TM<Q, S> {
    pub fn new(program: Program<Q, S>, tape: Tape<S>) -> Self
    where
        Q: Clone + Default,
        S: Default,
    {
        let state = program.initial_state().cloned();
        TM {
            program,
            state,
            tape,
        }
    }
    /// Returns an immutable reference to the [program](Program)
    pub const fn program(&self) -> &Program<Q, S> {
        &self.program
    }
    /// Creates a new instance of a [head](Head) from references to the current state and symbol;
    pub fn read(&self) -> Option<Head<&'_ Q, &'_ S>> {
        self.tape()
            .read()
            .ok()
            .map(|symbol| Head::new(self.state(), symbol))
    }
    /// Returns an instance of the [state](State) with an immutable
    /// reference to the internal data
    pub fn state(&self) -> State<&'_ Q> {
        self.state.to_ref()
    }
    /// Returns an instance of the [state](State) with a mutable
    /// reference to the internal data
    pub fn state_mut(&mut self) -> State<&'_ mut Q> {
        self.state.to_mut()
    }
    /// Returns an immutable reference to the [tape](StdTape)
    pub const fn tape(&self) -> &Tape<S> {
        &self.tape
    }
    /// Returns a mutable reference to the [tape](StdTape)
    pub fn tape_mut(&mut self) -> &mut Tape<S> {
        &mut self.tape
    }
    /// Runs the program until the
    ///
    /// The program will continue to run until the current state is a halt state.
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "run", target = "turing")
    )]
    pub fn execute(mut self) -> Result<(), Error>
    where
        Q: Clone + PartialEq + 'static,
        S: Symbolic,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!("Executing the program...");
        loop {
            #[cfg(feature = "tracing")]
            tracing::info!("{tape}", tape = self.tape());
            match self.process() {
                Some(_) => {
                    if self.state.is_halt() {
                        return Ok(());
                    } else {
                        continue;
                    }
                }
                None => {
                    return Err(Error::runtime_error(""));
                }
            }
        }
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "process", target = "turing")
    )]
    fn process(&mut self) -> Option<Head<&'_ Q, &'_ S>>
    where
        Q: Clone + PartialEq,
        S: Clone + PartialEq,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!("Processing the current instruction...");
        // Get the first instruction for the current head
        if let Some(Tail {
            direction,
            state,
            symbol,
        }) = self.program.get_head_ref(self.read()?)
        {
            //
            self.tape.update(direction, symbol.clone());
            self.state = state.cloned();
            return Some(Head::new(state, symbol));
        }
        unreachable!("No instruction found for the current head")
    }
}

impl<Q, S> core::iter::Iterator for TM<Q, S>
where
    Q: Clone + PartialEq + 'static,
    S: Clone + PartialEq,
{
    type Item = Head<Q, S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.process().map(|i| i.cloned())
    }
}
