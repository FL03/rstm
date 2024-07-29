/*
    Appellation: tm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::model::Turing;

pub(crate) mod model;

use crate::prelude::{Error, Head, Symbolic, Tape};
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
    /// Returns an instance of the [state](State) with an immutable
    pub fn set_state(&mut self, state: State<Q>) {
        self.state = state;
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
        tracing::instrument(skip_all, name = "run", target = "fsm")
    )]
    pub fn execute(mut self) -> Result<(), Error>
    where
        Q: Clone + PartialEq,
        S: Symbolic,
    {
        #[cfg(feature = "tracing")]
        tracing::info!("Running the program...");
        loop {
            #[cfg(feature = "tracing")]
            tracing::info!("{}", &self.tape);
            match self.next() {
                Some(_) => {
                    // if self.current_state().is_halt() {
                    //     return Ok(());
                    // }
                    continue;
                }
                None => {
                    return Err(Error::unknown("Runtime Error"));
                }
            }
        }
    }
}

impl<Q, S> core::iter::Iterator for TM<Q, S>
where
    Q: Clone + PartialEq,
    S: Symbolic,
{
    type Item = Head<Q, S>;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "step", target = "fsm")
    )]
    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(feature = "tracing")]
        tracing::info!("Stepping...");
        // Get the first instruction for the current head
        if let Some(tail) = self.program.get_ref(self.read()?) {
            self.state = self.tape.update_inplace(tail.cloned());
            return Some(tail.cloned().into_head());
        }
        None
    }
}
