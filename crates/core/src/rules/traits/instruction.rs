/*
    Appellation: instruction <module>
    Created At: 2026.01.15:11:50:07
    Contrib: @FL03
*/
use crate::rules::{Head, RawHead, RawTail, Rule, Tail};
use rstm_state::{RawState, State};

/// The [`Instruction`] trait establishes the base interface for all compatible rules for the
/// automata.
pub trait Instruction<Q, A>
where
    Q: RawState,
{
    /// the type of head used by the instruction
    type Head: RawHead<State = Q, Symbol = A>;
    /// the type of tail used by the instruction
    type Tail: RawTail<State = Q, Symbol = A>;
    /// returns a reference to the head of the instruction
    fn head(&self) -> &Self::Head;
    /// returns a reference to the tail of the instruction
    fn tail(&self) -> &Self::Tail;
    /// returns a reference to the current state
    fn current_state(&self) -> &State<Q>;
    /// returns a reference to the current symbol
    fn symbol(&self) -> &A;
    /// returns the direction of the tail
    fn direction(&self) -> crate::Direction {
        self.tail().direction()
    }
    /// returns a reference to the next state
    fn next_state(&self) -> &State<Q>;
    /// returns a reference to the next symbol
    fn next_symbol(&self) -> &A;
}

pub trait InstructionMut<Q, A>: Instruction<Q, A>
where
    Q: RawState,
{
    /// returns a mutable reference to the head of the instruction
    fn head_mut(&mut self) -> &mut Self::Head;
    /// returns a mutable reference to the tail of the instruction
    fn tail_mut(&mut self) -> &mut Self::Tail;
    /// returns a mutable reference to the current state
    fn current_state_mut(&mut self) -> &mut State<Q>;
    /// returns a mutable reference to the current symbol
    fn symbol_mut(&mut self) -> &mut A;
    /// returns a mutable reference to the next state
    fn next_state_mut(&mut self) -> &mut State<Q>;
    /// returns a mutable reference to the next symbol
    fn next_symbol_mut(&mut self) -> &mut A;
}

/*
 ************* Implementations *************
*/

impl<Q, A> Instruction<Q, A> for (Head<Q, A>, Tail<Q, A>)
where
    Q: RawState,
{
    type Head = Head<Q, A>;
    type Tail = Tail<Q, A>;

    fn head(&self) -> &Self::Head {
        &self.0
    }

    fn tail(&self) -> &Self::Tail {
        &self.1
    }

    fn current_state(&self) -> &State<Q> {
        self.head().state()
    }

    fn symbol(&self) -> &A {
        self.head().symbol()
    }

    fn next_state(&self) -> &State<Q> {
        self.tail().next_state()
    }

    fn next_symbol(&self) -> &A {
        self.tail().write_symbol()
    }
}

impl<Q, A> Instruction<Q, A> for Rule<Q, A>
where
    Q: RawState,
{
    type Head = Head<Q, A>;
    type Tail = Tail<Q, A>;

    fn head(&self) -> &Self::Head {
        &self.head
    }

    fn tail(&self) -> &Self::Tail {
        &self.tail
    }

    fn current_state(&self) -> &State<Q> {
        self.head().state()
    }

    fn symbol(&self) -> &A {
        self.head().symbol()
    }

    fn next_state(&self) -> &State<Q> {
        self.tail().next_state()
    }

    fn next_symbol(&self) -> &A {
        self.tail().write_symbol()
    }
}

impl<Q, A> InstructionMut<Q, A> for Rule<Q, A>
where
    Q: RawState,
{
    fn head_mut(&mut self) -> &mut Self::Head {
        &mut self.head
    }

    fn tail_mut(&mut self) -> &mut Self::Tail {
        &mut self.tail
    }

    fn current_state_mut(&mut self) -> &mut State<Q> {
        self.head_mut().state_mut()
    }

    fn symbol_mut(&mut self) -> &mut A {
        self.head_mut().symbol_mut()
    }

    fn next_state_mut(&mut self) -> &mut State<Q> {
        self.tail_mut().state_mut()
    }

    fn next_symbol_mut(&mut self) -> &mut A {
        self.tail_mut().symbol_mut()
    }
}

impl<Q, A> InstructionMut<Q, A> for (Head<Q, A>, Tail<Q, A>)
where
    Q: RawState,
{
    fn head_mut(&mut self) -> &mut Self::Head {
        &mut self.0
    }

    fn tail_mut(&mut self) -> &mut Self::Tail {
        &mut self.1
    }

    fn current_state_mut(&mut self) -> &mut State<Q> {
        self.head_mut().state_mut()
    }

    fn symbol_mut(&mut self) -> &mut A {
        self.head_mut().symbol_mut()
    }

    fn next_state_mut(&mut self) -> &mut State<Q> {
        self.tail_mut().state_mut()
    }

    fn next_symbol_mut(&mut self) -> &mut A {
        self.tail_mut().symbol_mut()
    }
}
