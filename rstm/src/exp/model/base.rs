/*
    Appellation: tm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Direction, Error, Head, Rule, StdTape, Symbolic, Tail};
use crate::rules::Ruleset;
use crate::state::{halt::HaltState, RawState, State};

/// # Turing Machine ([StdTm])
///
/// The Turing Machine is a mathematical model of computation that uses a set of rules to determine
/// how a machine should manipulate a tape. The machine can read, write, and move linearly across the tape.
/// Each pre-defined rule maps a head, consisting of a state and symbol, to a new state and symbol along with a direction.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct StdTM<Q, A> {
    pub(crate) program: Ruleset<Q, A>,
    pub(crate) state: State<Option<Q>>,
    pub(crate) tape: StdTape<A>,
}

impl<Q, A> StdTM<Q, A> {
    pub fn new(rules: impl IntoIterator<Item = Rule<Q, A>>, tape: StdTape<A>) -> Self
    where
        A: Default,
    {
        StdTM {
            program: Ruleset::from_iter(rules),
            state: State::none(),
            tape,
        }
    }
    /// Returns an immutable reference to the [program](Program)
    pub const fn program(&self) -> &Ruleset<Q, A> {
        &self.program
    }
    /// Creates a new instance of a [head](Head) from references to the current state and symbol;
    pub fn read(&self) -> Option<Head<&'_ Option<Q>, &'_ A>> {
        self.tape()
            .read()
            .ok()
            .map(|symbol| Head::new(self.state(), symbol))
    }

    pub fn state(&self) -> State<&Option<Q>> {
        self.state.to_ref()
    }
    /// Returns an immutable reference to the [tape](StdTape)
    pub const fn tape(&self) -> &StdTape<A> {
        &self.tape
    }
    /// Returns a mutable reference to the [tape](StdTape)
    pub fn tape_mut(&mut self) -> &mut StdTape<A> {
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
        A: Symbolic,
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

    fn handle(&mut self, direction: Direction, state: State<Option<Q>>, symbol: A) {
        self.tape.update(direction, symbol);
        self.state = state.into();
    }

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "process", target = "turing")
    )]
    fn process(&mut self) -> Option<Head<&'_ Q, &'_ A>>
    where
        Q: Clone + PartialEq,
        A: Clone + PartialEq,
    {
        #[cfg(feature = "tracing")]
        tracing::trace!("Processing the current instruction...");
        let csymbol = self.tape.read().ok()?;
        if let Some(cstate) = self.state().get() {
            if let Some(Tail {
                direction,
                state,
                symbol,
            }) = self.program.get(State(cstate), csymbol)
            {
                //
                self.tape.update(*direction, symbol.clone());
                self.state = state.clone().map(Some);
                return Some(Head::new(State(state), symbol));
            }
            unreachable!("No instruction found for the current head")
        } else {
            #[cfg(feature = "tracing")]
            tracing::trace!("Head is halted, terminating execution...");
            return None;
        }
        
    }
}
