/*
    Appellation: exec <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Actor, Engine, Handle};
use crate::state::{RawState, State};
use crate::{Head, RuleSet, Symbolic, Tail};

/// The [Executor] handles the execution of a given program. The structure works as an
/// iterator, where each iteration represents a step in the program. The executor is
/// responsible for reading the current symbol at the head of the tape, executing the program,
/// and updating the tape accordingly. The executor will continue to iterate until the actor
/// is halted or the program is completed.
pub struct Executor<Q, S>
where
    Q: RawState,
{
    /// the actor that will be executing the program
    pub(crate) actor: Actor<Q, S>,
    /// the program being executed
    pub(crate) program: RuleSet<Q, S>,
    /// the number of steps taken by the actor
    pub(crate) steps: usize,
}

impl<Q, S> Executor<Q, S>
where
    Q: RawState,
{
    pub(crate) fn new(actor: Actor<Q, S>, program: RuleSet<Q, S>) -> Self {
        Self {
            actor,
            program,
            steps: 0,
        }
    }

    pub fn from_actor(actor: Actor<Q, S>) -> Self
    where
        Q: Default,
    {
        Self {
            actor,
            program: RuleSet {
                initial_state: Default::default(),
                rules: Vec::new(),
            },
            steps: 0,
        }
    }
    /// Load a program into the executor
    pub fn load(self, program: RuleSet<Q, S>) -> Self {
        Executor { program, ..self }
    }
    /// returns a reference to the actor
    pub const fn actor(&self) -> &Actor<Q, S> {
        &self.actor
    }
    /// returns a mutable reference to the actor
    pub const fn actor_mut(&mut self) -> &mut Actor<Q, S> {
        &mut self.actor
    }
    /// returns a copy of the current steps
    pub const fn steps(&self) -> usize {
        self.steps
    }
    /// returns a mutable reference to the current steps
    pub const fn current_state(&self) -> &State<Q> {
        self.actor().state()
    }

    pub fn get_rule<K>(&self, state: State<&Q>, symbol: &S) -> Option<&Tail<Q, S>>
    where
        Q: Eq + core::hash::Hash,
        S: Eq + core::hash::Hash,
    {
        self.program.get(state, symbol)
    }
    /// Reads the current symbol at the head of the tape
    pub fn read(&self) -> crate::Result<Head<&Q, &S>> {
        self.actor().read()
    }
    /// Reads the current symbol at the head of the tape
    pub fn read_uninit(&self) -> Head<&Q, core::mem::MaybeUninit<&S>> {
        if let Ok(Head { state, symbol }) = self.read() {
            Head {
                state,
                symbol: core::mem::MaybeUninit::new(symbol),
            }
        } else {
            Head {
                state: self.current_state().view(),
                symbol: core::mem::MaybeUninit::uninit(),
            }
        }
    }
    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "run", target = "actor")
    )]
    pub fn run(&mut self) -> crate::Result<()>
    where
        Q: 'static + RawState + Clone + PartialEq,
        S: crate::Symbolic,
    {
        #[cfg(feature = "tracing")]
        tracing::info!("Running the program...");
        while let Some(_h) = self.next() {
            #[cfg(feature = "tracing")]
            tracing::info!("Executing step: {head:?}", head = _h);
        }
        Ok(())
    }

    fn _handle_tail(&mut self, tail: Tail<Q, S>) -> crate::Result<Head<Q, S>>
    where
        Q: RawState + Clone + PartialEq,
        S: crate::Symbolic,
    {
        // process the instruction
        let next = tail.as_head().cloned();
        // process the instruction
        let _prev = self.handle(tail);
        // return the head
        Ok(next)
    }
}

impl<D, Q, S> Handle<D> for Executor<Q, S>
where
    Q: RawState + Clone + PartialEq,
    S: crate::Symbolic,
    Actor<Q, S>: Handle<D>,
{
    type Output = <Actor<Q, S> as Handle<D>>::Output;

    fn handle(&mut self, args: D) -> Self::Output {
        self.actor_mut().handle(args)
    }
}

impl<Q, S> Engine<Q, S> for Executor<Q, S>
where
    Q: 'static + RawState + Clone + PartialEq,
    S: crate::Symbolic,
{
    fn load<I>(&mut self, program: I)
    where
        I: IntoIterator<Item = crate::Rule<Q, S>>,
    {
        self.program.rules.clear();
        self.program.extend(program);
    }

    fn run(&mut self) -> Result<(), crate::Error> {
        Executor::run(self)
    }
}

impl<Q, S> Iterator for Executor<Q, S>
where
    Q: 'static + RawState + Clone + PartialEq,
    S: Symbolic,
{
    type Item = Head<Q, S>;

    #[cfg_attr(
        feature = "tracing",
        tracing::instrument(skip_all, name = "next", target = "actor")
    )]
    fn next(&mut self) -> Option<Self::Item> {
        // increment the number of steps taken
        self.steps += 1;
        #[cfg(feature = "tracing")]
        tracing::info!("{tape:?}", tape = self.actor());
        // check if the actor is halted
        if self.actor.is_halted() {
            #[cfg(feature = "tracing")]
            tracing::warn!("Detected a halted state; terminating the program...");
            return None;
        }
        // read the tape
        let head = if let Ok(cur) = self.read() {
            cur
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!(
                "[Index Error] the current position ({pos}) of the head is out of bounds, assuming the symbol to be its default value...",
                pos = self.actor().head().symbol()
            );
            Head {
                state: self.actor().state().view(),
                symbol: &S::default(),
            }
        };
        // execute the program
        if let Some(tail) = self.program.get(head.state, head.symbol).cloned() {
            // process the instruction
            let next = tail.clone().into_head();
            // process the instruction
            let _prev = self.handle(tail);
            // return the head
            return Some(next);
        } else {
            #[cfg(feature = "tracing")]
            tracing::error!("No symbol found at {}", self.actor.position());
            panic!("No symbol found at {}", self.actor.position());
        }
    }
}
