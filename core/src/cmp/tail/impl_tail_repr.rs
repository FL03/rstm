/*
    Appellation: impl_tail_repr <module>
    Created At: 2025.09.04:19:35:05
    Contrib: @FL03
*/
use super::{Tail, TailMut, TailRef};
use rstm_state::RawState;
impl<'a, Q, S> TailRef<'a, Q, S>
where
    Q: RawState,
{
    /// returns a new [`Tail`] with cloned elements
    pub fn cloned(&self) -> Tail<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Tail {
            direction: self.direction,
            next_state: self.next_state.cloned(),
            write_symbol: self.write_symbol.clone(),
        }
    }
    /// returns a new [`Tail`] with copied elements
    pub fn copied(&self) -> Tail<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Tail {
            direction: self.direction,
            next_state: self.next_state.copied(),
            write_symbol: *self.write_symbol,
        }
    }
}

impl<'a, Q, S> TailMut<'a, Q, S>
where
    Q: RawState,
{
    /// returns a new [`Tail`] with cloned elements
    pub fn cloned(&self) -> Tail<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Tail {
            direction: self.direction,
            next_state: self.next_state.cloned(),
            write_symbol: self.write_symbol.clone(),
        }
    }
    /// returns a new [`Tail`] with copied elements
    pub fn copied(&self) -> Tail<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Tail {
            direction: self.direction,
            next_state: self.next_state.copied(),
            write_symbol: *self.write_symbol,
        }
    }
}
