/*
    Appellation: transition <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::{Direction, Head};
use crate::State;

/// [Transition] is a type representing the tail of a Turing machine;
///
/// Formally, a tail is defined to be a 3-tuple (direction, state, symbol). Considering the
/// head of a machine is a 2-tuple (state, symbol), the tail simply extends the head with a
/// direction.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
pub struct Transition<Q, S> {
    pub direction: Direction,
    pub head: Head<Q, S>,
}

impl<Q, S> Transition<Q, S> {
    pub fn new(direction: Direction, state: State<Q>, symbol: S) -> Self {
        Self {
            direction,
            head: Head::new(state, symbol),
        }
    }

    pub fn with_direction(self, direction: Direction) -> Self {
        Self { direction, ..self }
    }

    pub fn with_state(self, state: State<Q>) -> Self {
        Self {
            head: self.head.with_state(state),
            ..self
        }
    }

    pub fn with_symbol(self, symbol: S) -> Self {
        Self {
            head: self.head.with_symbol(symbol),
            ..self
        }
    }

    pub fn as_head_ref(&self) -> Head<&Q, &S> {
        self.head.to_ref()
    }

    pub fn as_mut_head(&mut self) -> Head<&mut Q, &mut S> {
        self.head.to_mut()
    }

    pub const fn head(&self) -> &Head<Q, S> {
        &self.head
    }

    pub fn head_mut(&mut self) -> &mut Head<Q, S> {
        &mut self.head
    }

    pub fn state(&self) -> State<&'_ Q> {
        self.head.state()
    }

    pub fn state_mut(&mut self) -> State<&'_ mut Q> {
        self.head.state_mut()
    }

    pub fn symbol(&self) -> &S {
        self.head.symbol()
    }

    pub fn symbol_mut(&mut self) -> &mut S {
        self.head.symbol_mut()
    }

    pub fn to_ref(&self) -> Transition<&'_ Q, &'_ S> {
        Transition {
            direction: self.direction,
            head: self.head.to_ref(),
        }
    }

    pub fn to_mut(&mut self) -> Transition<&'_ mut Q, &'_ mut S> {
        Transition {
            direction: self.direction,
            head: self.head.to_mut(),
        }
    }
}

impl<'a, Q, S> Transition<&'a Q, &'a S> {
    pub fn cloned(&self) -> Transition<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Transition {
            direction: self.direction,
            head: self.head.cloned(),
        }
    }

    pub fn copied(&self) -> Transition<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Transition {
            direction: self.direction,
            head: self.head.copied(),
        }
    }
}

impl<'a, Q, S> Transition<&'a mut Q, &'a mut S> {
    pub fn cloned(&self) -> Transition<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Transition {
            direction: self.direction,
            head: self.head.cloned(),
        }
    }

    pub fn copied(&self) -> Transition<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Transition {
            direction: self.direction,
            head: self.head.copied(),
        }
    }
}

impl<Q, S> core::fmt::Debug for Transition<Q, S>
where
    Q: core::fmt::Debug,
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Transition")
            .field(&self.direction)
            .field(&self.head.state)
            .field(&self.head.symbol)
            .finish()
    }
}

impl<Q, S> core::fmt::Display for Transition<Q, S>
where
    Q: core::fmt::Display,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{dir}({head})", dir = self.direction, head = self.head)
    }
}
