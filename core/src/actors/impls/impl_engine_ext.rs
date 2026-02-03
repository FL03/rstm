/*
    Appellation: impl_engine_ext <module>
    Created At: 2026.01.17:19:54:00
    Contrib: @FL03
*/

use crate::actors::engine_base::EngineBase;
use crate::actors::{Driver, Executor};
use crate::programs::Program;
use crate::rules::Head;
use rstm_state::{Halting, RawState};
use rstm_traits::{Reader, Symbolic, TryExecute, TryStep};

impl<'a, D, Q, A> Reader<A> for EngineBase<D, Q, A>
where
    Q: RawState,
    D: Driver<Q, A>,
{
    type Error = crate::Error;

    fn read(&self) -> Result<&A, Self::Error> {
        self.read()
    }
}

impl<'a, D, Q, A, X, Y, E> TryExecute<X> for EngineBase<D, Q, A>
where
    Q: RawState,
    D: Driver<Q, A> + TryExecute<X, Output = Y, Error = E>,
{
    type Error = E;
    type Output = Y;

    fn try_execute(&mut self, args: X) -> Result<Self::Output, Self::Error> {
        self.driver_mut().try_execute(args)
    }
}

impl<D, Q, A> Executor<Q, A> for EngineBase<D, Q, A>
where
    D: Driver<Q, A>,
    Q: Halting + RawState,
    Self: TryStep<Output = Head<Q, A>, Error = crate::Error>,
{
    type Driver = D;

    seal! {}

    fn load(&mut self, program: Program<Q, A>) {
        self.program = Some(program);
    }

    fn run(&mut self) -> crate::Result<()> {
        self.run()
    }
}

impl<Q, A> TryStep for EngineBase<Head<Q, usize>, Q, A>
where
    A: Symbolic,
    Q: RawState + Clone + PartialEq,
{
    type Error = crate::Error;
    type Output = Head<Q, A>;

    fn try_step(&mut self) -> Result<Self::Output, Self::Error> {
        #[cfg(feature = "tracing")]
        tracing::info! { "{}", self.print() };
        // if the output tape is empty, initialize it from the driver's tape
        if self.tape().is_empty() {
            #[cfg(feature = "tracing")]
            tracing::error! { "Output tape is empty; initializing from the input tape..." };
            return Err(crate::Error::TapeIsEmpty);
        }
        // read the tape
        let Head {
            state,
            symbol: &pos,
        } = self.driver.view();
        let current_symbol = &self.tape[pos];
        // get a reference to the program
        if let Some(program) = self.program() {
            // use the program to find a tail for the current head
            let tail = program
                .find_tail(state, current_symbol)
                .ok_or(crate::Error::NoRuleFound)?
                .clone();
            // increment the steps
            self.next_cycle();
            // process the instruction
            let step = self.driver.step(tail);
            // apply the step
            return step.shift(&mut self.tape);
        }
        // if there is no program loaded, return an error
        #[cfg(feature = "tracing")]
        tracing::error!("No program loaded; cannot execute step.");
        Err(crate::Error::NoProgram)
    }
}

impl<'a, D, Q, A> Iterator for EngineBase<D, Q, A>
where
    Q: 'static + Halting + RawState + Clone + PartialEq,
    A: Symbolic,
    D: Driver<Q, A>,
    Self: TryStep<Output = Head<Q, A>>,
{
    type Item = Head<Q, A>;

    fn next(&mut self) -> Option<Self::Item> {
        self.try_step().ok()
    }
}
