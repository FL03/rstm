/*
    Appellation: tm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::rules::Ruleset;
use crate::state::{halt::HaltState, State};
use crate::{Error, Head, StdTape, Symbolic, Tail};

/// # Turing Machine ([StdTm])
///
/// The Turing Machine is a mathematical model of computation that uses a set of rules to determine
/// how a machine should manipulate a tape. The machine can read, write, and move linearly across the tape.
/// Each pre-defined rule maps a head, consisting of a state and symbol, to a new state and symbol along with a direction.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct StdTM<Q = String, S = char> {
    pub(crate) program: Ruleset<Q, S>,
    pub(crate) state: HaltState<Q>,
    pub(crate) tape: StdTape<S>,
}

impl<Q, S> StdTM<Q, S> {
    pub fn new(program: Ruleset<Q, S>, tape: StdTape<S>) -> Self
    where
        Q: Clone + Default,
        S: Default,
    {
        let state = program
            .initial_state()
            .map(|q| q.cloned())
            .unwrap_or_default();
        StdTM {
            program,
            state: HaltState::state(state),
            tape,
        }
    }
    /// Returns an immutable reference to the [program](Program)
    pub const fn program(&self) -> &Ruleset<Q, S> {
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
        self.state.as_state()
    }
    /// Returns an instance of the [state](State) with a mutable
    /// reference to the internal data
    pub fn state_mut(&mut self) -> State<&'_ mut Q> {
        self.state.as_mut_state()
    }
    /// Returns an immutable reference to the [tape](StdTape)
    pub const fn tape(&self) -> &StdTape<S> {
        &self.tape
    }
    /// Returns a mutable reference to the [tape](StdTape)
    pub fn tape_mut(&mut self) -> &mut StdTape<S> {
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
        }) = self.program.get_ref(self.read()?)
        {
            //
            self.tape.update(direction, symbol.clone());
            self.state = state.cloned().into();
            return Some(Head::new(state, symbol));
        }
        unreachable!("No instruction found for the current head")
    }
}

impl<Q, S> core::iter::Iterator for StdTM<Q, S>
where
    Q: Clone + PartialEq + 'static,
    S: Clone + PartialEq,
{
    type Item = Head<Q, S>;

    fn next(&mut self) -> Option<Self::Item> {
        self.process().map(|i| i.cloned())
    }
}
