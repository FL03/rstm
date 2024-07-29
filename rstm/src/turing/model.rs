/*
    Appellation: tm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::{Error, Head, StdTape, Symbolic};
use crate::rules::{Instruction, Program};
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
    pub(crate) tape: StdTape<S>,
}

impl<Q, S> TM<Q, S> {
    pub fn new(
        State(state): State<Q>,
        instructions: impl IntoIterator<Item = Instruction<Q, S>>,
        tape: StdTape<S>,
    ) -> Self
    where
        Q: Clone,
        S: Clone + Default,
    {
        let state = State(state);
        let program = Program::new(state.clone()).with_instructions(instructions);
        TM { program, state, tape }
    }

    pub fn head(&self) -> Head<&'_ Q, &'_ S>
    where
        Q: Clone,
        S: Clone,
    {
        let state = self.current_state();
        let symbol = self.tape().read().unwrap();
        Head::new(state, symbol)
    }

    pub const fn program(&self) -> &Program<Q, S> {
        &self.program
    }

    pub fn current_state(&self) -> State<&'_ Q> {
        self.state.to_ref()
    }

    pub fn set_state(&mut self, state: State<Q>) {
        self.state = state;
    }

    pub const fn tape(&self) -> &StdTape<S> {
        &self.tape
    }

    pub fn tape_mut(&mut self) -> &mut StdTape<S> {
        &mut self.tape
    }
}

// #[cfg(feature = "std")]
impl<Q, S> TM<Q, S>
where
    Q: Clone + PartialEq,
    S: Symbolic,
{
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "step", target = "fsm")
    )]
    pub fn step_inplace(&mut self) -> Result<Head<Q, S>, Error> {
        #[cfg(feature = "tracing")]
        tracing::info!("Stepping...");
        let prog = self.program().clone();
        // Create a new head from the current state and symbol
        let head = self.head().cloned();
        // Get the first instruction for the current head
        if let Some(&tail) = prog.get_head(&head).first() {
            let nxt = self.tape.update_inplace(tail.clone());
            self.set_state(nxt);
            return Ok(tail.as_head().cloned());
        }
        Err(Error::state_not_found(""))
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "run", target = "fsm")
    )]
    pub fn run(mut self) -> Result<(), Error> {
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

    fn next(&mut self) -> Option<Self::Item> {
        self.step_inplace().ok()
    }
}
