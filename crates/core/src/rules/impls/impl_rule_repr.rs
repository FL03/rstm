/*
    Appellation: impl_rule_repr <module>
    Created At: 2026.01.14:23:11:59
    Contrib: @FL03
*/
use crate::rules::rule::Rule;
use crate::rules::{Head, Tail};
use rstm_state::RawState;

impl<'a, Q, A, R, B> Rule<&'a Q, &'a A, &'a R, &'a B>
where
    Q: RawState,
    R: RawState,
{
    /// returns a new instance of the [`Rule`] with cloned elements
    pub fn cloned(&self) -> Rule<Q, A, R, B>
    where
        Q: Clone,
        A: Clone,
        R: Clone,
        B: Clone,
    {
        Rule {
            head: self.head.cloned(),
            tail: self.tail.cloned(),
        }
    }
    /// returns a new instance of the [`Rule`] with copied elements
    pub const fn copied(&self) -> Rule<Q, A, R, B>
    where
        Q: Copy,
        A: Copy,
        R: Copy,
        B: Copy,
    {
        Rule {
            head: self.head.copied(),
            tail: self.tail.copied(),
        }
    }
}

impl<'a, Q, A> Rule<Q, usize, Q, A>
where
    Q: RawState,
{
    /// this method shifts the head along the tape, returning a head containing the previous
    /// state and symbol.
    ///
    /// **note**: this method **does not** check if the current nor the next state is halted,
    /// it is up to the caller to establishing halting.
    pub fn shift(&mut self, tape: &mut [A]) -> Option<Head<Q, A>>
    where
        A: Clone,
        Q: Clone,
    {
        let Tail {
            next_state,
            direction,
            write_symbol,
        } = self.tail.clone();
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
