/*
    Appellation: moving_head <module>
    Created At: 2025.12.19:14:43:21
    Contrib: @FL03
*/
use crate::{Head, Tail};
use rstm_state::RawState;

/// [`HeadStep`] is a structure responsible for managing the step operation of a moving head
/// in a Turing machine simulation.
pub struct HeadStep<'a, Q, A, R = Q, B = A>
where
    Q: RawState,
    R: RawState,
{
    pub(crate) head: &'a mut Head<Q, A>,
    pub(crate) tail: Tail<R, B>,
}

impl<'a, Q1, Q2, A, B> HeadStep<'a, Q1, A, Q2, B>
where
    Q1: RawState,
    Q2: RawState,
{
    /// a private constructor used to create a new instance of the [`HeadStep`]
    pub(crate) const fn new(head: &'a mut Head<Q1, A>, tail: Tail<Q2, B>) -> Self {
        Self { head, tail }
    }
}
impl<'a, Q, A> HeadStep<'a, Q, A>
where
    Q: RawState,
{
    #[inline]
    /// apply the step, mutating the head according to the tail's instructions before
    /// returning the previous head value.
    pub fn apply(self) -> Head<Q, A> {
        let Tail {
            next_state,
            write_symbol,
            ..
        } = self.tail;
        // replace the head with the next state and symbol, returning the previous head
        self.head.replace(next_state, write_symbol)
    }

    #[inline]
    /// this method performs the step operation onto the tape, assuming the head's symbol to be  
    pub fn move_along(self, tape: &mut [A], pos: &mut usize) -> Head<Q, A>
    where
        A: Clone,
    {
        tape[*pos] = self.tail.write_symbol.clone();
        *pos += self.tail.direction;
        self.apply()
    }
}

impl<'a, Q, A> HeadStep<'a, Q, usize, Q, A>
where
    Q: RawState,
{
    pub fn apply_directional(self) -> Head<Q, usize> {
        let Tail {
            next_state,
            direction,
            ..
        } = self.tail;
        // replace the head with the next state and symbol, returning the previous head
        let prev_state = self.head.replace_state(next_state);
        let prev_symbol = self.head.symbol;
        self.head.symbol += direction;
        Head {
            state: prev_state,
            symbol: prev_symbol,
        }
    }
    /// this method performs the step operation onto the tape, assuming the head's symbol to be  
    /// an usize value representing a position on the tape.
    pub fn move_along_tape(self, tape: &mut [A], pos: &mut usize) -> Head<Q, usize>
    where
        A: Clone,
    {
        tape[*pos] = self.tail.write_symbol.clone();
        self.head.symbol += self.tail.direction;
        self.apply_directional()
    }
}
