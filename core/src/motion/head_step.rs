/*
    Appellation: moving_head <module>
    Created At: 2025.12.19:14:43:21
    Contrib: @FL03
*/
use crate::{Head, Tail};
use rstm_state::RawState;

/// [`HeadStep`] is a structure responsible for managing the step operation of a moving head
/// in a Turing machine simulation.
pub struct HeadStep<'a, Q1, A1, Q2 = Q1, A2 = A1>
where
    Q1: RawState,
    Q2: RawState,
{
    pub(crate) head: &'a mut Head<Q1, A1>,
    pub(crate) tail: Tail<Q2, A2>,
}
/// the standard implementation of [`HeadStep`] focuses on instances where the head and tail
/// share the same type-space; meaning `Head<Q, A>` and `Tail<Q, A>` types are being utilized.
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

/// this implementation of the [`HeadStep`] is specifically designed for scenarios where the
/// head's symbol is used to define the position
impl<'a, Q, A> HeadStep<'a, Q, usize, Q, A>
where
    Q: RawState,
{
    #[inline]
    /// this method shifts the head along the tape, returning a head containing the previous
    /// state and symbol.
    ///
    /// **note**: this method **does not** check if the current nor the next state is halted,
    /// it is up to the caller to establishing halting.
    pub fn shift(self, tape: &mut [A]) -> Option<Head<Q, A>>
    where
        A: Clone,
    {
        let Tail {
            next_state,
            direction,
            write_symbol,
        } = self.tail;
        if let Some(sym) = tape.get(self.head.symbol).cloned() {
            // update the tape at the head's current position
            tape[self.head.symbol] = write_symbol;
            // update the head position based on the tail's direction
            self.head.symbol += direction;
            // reconstruct the previous head
            let prev = Head {
                state: self.head.replace_state(next_state),
                symbol: sym,
            };
            return Some(prev);
        }
        #[cfg(feature = "tracing")]
        tracing::error!(
            "The position of the head ({}) is out of tape bounds for a tape of length {}",
            self.head.symbol,
            tape.len()
        );
        // return None if the head's position is out of bounds
        None
    }
}
