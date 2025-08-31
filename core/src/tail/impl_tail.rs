/*
    Appellation: impl_tail <module>
    Created At: 2025.08.30:23:58:26
    Contrib: @FL03
*/
use super::Tail;
use rstm_state::RawState;

impl<'a, Q, S> Tail<&'a Q, &'a S>
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

impl<'a, Q, S> Tail<&'a mut Q, &'a mut S>
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

impl<Q, S> core::fmt::Debug for Tail<Q, S>
where
    Q: RawState,
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Tail")
            .field(&self.direction)
            .field(&self.next_state)
            .field(&self.write_symbol)
            .finish()
    }
}

impl<Q, S> core::fmt::Display for Tail<Q, S>
where
    Q: RawState,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ direction: {}, state: {:?}, symbol: {} }}",
            self.direction, self.next_state, self.write_symbol
        )
    }
}
