/*
    Appellation: impl_tail_ops <module>
    Created At: 2025.12.19:13:21:00
    Contrib: @FL03
*/
use super::Tail;
use crate::head::Head;
use crate::types::Direction;
use rstm_state::State;

impl<Q, A> core::fmt::Debug for Tail<Q, A>
where
    Q: core::fmt::Debug,
    A: core::fmt::Debug,
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
    Q: core::fmt::Display,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{{ direction: {}, next_state: {}, write_symbol: {} }}",
            self.direction, self.next_state, self.write_symbol
        )
    }
}

impl<Q, A> From<(Direction, Head<Q, A>)> for Tail<Q, A> {
    fn from((direction, head): (Direction, Head<Q, A>)) -> Self {
        Tail {
            direction,
            next_state: head.state,
            write_symbol: head.symbol,
        }
    }
}

impl<Q, A> From<Tail<Q, A>> for (Direction, Head<Q, A>) {
    fn from(tail: Tail<Q, A>) -> Self {
        (
            tail.direction,
            Head {
                state: tail.next_state,
                symbol: tail.write_symbol,
            },
        )
    }
}

impl<Q, A> From<(Direction, State<Q>, A)> for Tail<Q, A> {
    fn from((direction, next_state, write_symbol): (Direction, State<Q>, A)) -> Self {
        Tail {
            direction,
            next_state,
            write_symbol,
        }
    }
}

impl<Q, A> From<(Direction, Q, A)> for Tail<Q, A> {
    fn from((direction, next_state, write_symbol): (Direction, Q, A)) -> Self {
        Tail {
            direction,
            next_state: State(next_state),
            write_symbol,
        }
    }
}

impl<Q, A> From<Tail<Q, A>> for (Direction, State<Q>, A) {
    fn from(tail: Tail<Q, A>) -> Self {
        (tail.direction, tail.next_state, tail.write_symbol)
    }
}

impl<Q, A> PartialEq<(Direction, Head<Q, A>)> for Tail<Q, A>
where
    Q: PartialEq,
    A: PartialEq,
{
    fn eq(&self, (other_direction, other_head): &(Direction, Head<Q, A>)) -> bool {
        &self.direction == other_direction
            && &self.next_state == &other_head.state
            && &self.write_symbol == &other_head.symbol
    }
}

impl<Q, A> PartialEq<(Direction, State<Q>, A)> for Tail<Q, A>
where
    Q: PartialEq,
    A: PartialEq,
{
    fn eq(&self, (other_direction, other_state, other_symbol): &(Direction, State<Q>, A)) -> bool {
        &self.direction == other_direction
            && &self.next_state == other_state
            && &self.write_symbol == other_symbol
    }
}

impl<Q, A> PartialEq<Tail<Q, A>> for (Direction, State<Q>, A)
where
    Q: PartialEq,
    A: PartialEq,
{
    fn eq(&self, other: &Tail<Q, A>) -> bool {
        self.0 == other.direction && self.1 == other.next_state && self.2 == other.write_symbol
    }
}

impl<Q, A> PartialEq<(State<Q>, A)> for Tail<Q, A>
where
    Q: PartialEq,
    A: PartialEq,
{
    fn eq(&self, (other_state, other_symbol): &(State<Q>, A)) -> bool {
        &self.next_state == other_state && &self.write_symbol == other_symbol
    }
}

impl<Q, A> PartialEq<Tail<Q, A>> for (State<Q>, A)
where
    Q: PartialEq,
    A: PartialEq,
{
    fn eq(&self, other: &Tail<Q, A>) -> bool {
        &self.0 == &other.next_state && &self.1 == &other.write_symbol
    }
}
