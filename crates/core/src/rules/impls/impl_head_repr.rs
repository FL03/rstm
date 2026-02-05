/*
    Appellation: impl_head_repr <module>
    Created At: 2025.08.31:00:01:56
    Contrib: @FL03
*/
use crate::Direction;
use crate::actors::EngineBase;
use crate::rules::{Head, HeadMut, HeadRef};
use rstm_state::RawState;

impl<'a, Q, S> HeadRef<'a, Q, S>
where
    Q: RawState,
{
    /// returns a new [`Head`] with cloned elements
    pub fn cloned(&self) -> Head<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Head {
            state: self.state.cloned(),
            symbol: self.symbol.clone(),
        }
    }
    /// returns a new [`Head`] with copied elements
    pub const fn copied(&self) -> Head<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Head {
            state: self.state().copied(),
            symbol: *self.symbol,
        }
    }
}

impl<'a, Q, S> HeadMut<'a, Q, S>
where
    Q: RawState,
{
    /// returns a new [`Head`] with cloned elements
    pub fn cloned(&self) -> Head<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Head {
            state: self.state.cloned(),
            symbol: self.symbol.clone(),
        }
    }
    /// returns a new [`Head`] with copied elements
    pub fn copied(&self) -> Head<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Head {
            state: self.state.copied(),
            symbol: *self.symbol,
        }
    }
}

impl<Q> Head<Q, usize>
where
    Q: RawState,
{
    /// load the head into an engine loaded with the given program
    pub fn load<A>(self, program: crate::Program<Q, A>) -> EngineBase<Self, Q, A>
    where
        Q: PartialEq,
        A: PartialEq,
    {
        EngineBase::from_driver(self).with_program(program)
    }

    pub fn shift(self, direction: Direction) -> Self {
        Self {
            symbol: direction.apply_unsigned(self.symbol),
            ..self
        }
    }

    pub fn shift_inplace(&mut self, direction: Direction) {
        self.symbol = direction.apply_unsigned(self.symbol);
    }
}
