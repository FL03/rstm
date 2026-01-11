/*
    Appellation: moving_head <module>
    Created At: 2025.12.19:14:43:21
    Contrib: @FL03
*/
use crate::{Head, Tail};

/// The [`HeadStep`] is a generic structure working to emulate the behaviors of a Turing
/// machine with a moving head (TMH). The implementation essentially captures the behavior by
/// defining a single step operation that updates the head's state and symbol based on the
/// tail's instructions, returning the previous head value before the update.
pub struct HeadStep<'a, Q, A> {
    pub(crate) head: &'a mut Head<Q, A>,
    pub(crate) tail: Tail<Q, A>,
}

impl<'a, Q, A> HeadStep<'a, Q, A> {
    pub const fn new(head: &'a mut Head<Q, A>, tail: Tail<Q, A>) -> Self {
        Self { head, tail }
    }
    #[inline]
    /// executes a single step, mutating the head according to the tail's instructions before
    /// returning the previous head value
    pub fn step(self) -> Head<Q, A> {
        let next = self.tail.into_head();
        self.head.replace(next.state, next.symbol)
    }
    #[inline]
    /// executes a single step, mutating the head according to the tail's instructions before
    /// returning the previous head value
    pub fn step_on(self, tape: &mut [A], pos: &mut usize) -> Head<Q, A>
    where
        A: Clone,
    {
        tape[*pos] = self.tail.write_symbol.clone();
        *pos += self.tail.direction;
        self.step()
    }
}
