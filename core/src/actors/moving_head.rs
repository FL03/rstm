/*
    Appellation: moving_head <module>
    Created At: 2025.12.19:14:43:21
    Contrib: @FL03
*/
use crate::{Head, Tail};

/// The [`MovingHead`] is a generic structure working to emulate the behaviors of a Turing
/// machine with a moving head (TMH). The implementation essentially takes a mutable reference
/// to some head and mutates it according to the configured _tail_.
pub struct MovingHead<'a, Q, A> {
    pub(crate) head: &'a mut Head<Q, A>,
    pub(crate) tail: Tail<Q, A>,
}

impl<'a, Q, A> MovingHead<'a, Q, A> {
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
