/*
    Appellation: base <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Alphabet, Head, State};

pub struct ActorBase<Q, S>
where
    S: Alphabet,
{
    /// the head of the tape
    pub(crate) head: Head<Q, *mut S::Elem>,
    /// the input alphabet
    pub(crate) tape: S,
}

impl<A, Q, S> ActorBase<Q, S>
where
    S: Alphabet<Elem = A>,
{
    pub fn new(alpha: S, state: State<Q>, symbol: *mut A) -> Self {
        Self {
            head: Head { state, symbol },
            tape: alpha,
        }
    }
    /// Constructs a new [Actor] with the given state; assumes the tape is empty and the head
    /// is located at `0`.
    pub fn from_state(state: State<Q>) -> Self
    where
        S: Default,
    {
        Self {
            head: Head {
                state,
                symbol: core::ptr::null_mut(),
            },
            tape: S::default(),
        }
    }
    /// Consumes the current instance and returns a new instance with the given alphabet
    pub fn with_alpha(self, alpha: S) -> Self {
        Self {
            tape: alpha,
            ..self
        }
    }
    /// Consumes the current instance and returns a new instance with the given head
    pub fn with_head(self, head: Head<Q, *mut A>) -> Self {
        Self { head, ..self }
    }
    /// Consumes the current instance and returns a new instance with the given position
    pub fn with_ptr(self, symbol: *mut A) -> Self {
        Self {
            head: Head {
                symbol,
                ..self.head
            },
            ..self
        }
    }
    /// Consumes the current instance and returns a new instance with the given state
    pub fn with_state(self, state: State<Q>) -> Self {
        Self {
            head: Head { state, ..self.head },
            ..self
        }
    }
    /// Consumes the current instance and returns a new instance with the given state
    pub fn with_state_symbol(self, state: State<Q>, symbol: *mut S::Elem) -> Self {
        Self {
            head: Head { state, symbol },
            ..self
        }
    }
}
