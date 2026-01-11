/*
    Appellation: impl_tail_ops <module>
    Created At: 2025.12.19:13:21:00
    Contrib: @FL03
*/
use super::Tail;
use crate::head::Head;
use crate::state::State;
use crate::types::Direction;

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
            && &self.next_state == other_head.state()
            && &self.write_symbol == other_head.symbol()
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
