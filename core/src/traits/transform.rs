/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Direction, Head, State};

/// [TM]
pub trait TM<Q, A> {
    type Idx: Copy + core::ops::Add<Direction, Output = Self::Idx>;

    fn process(&mut self, direction: Direction, state: State<Q>, symbol: A) -> Head<Q, Self::Idx> {
        let pos = self.position();
        self.write(symbol);
        self.scope_mut().replace(state, pos + direction)
    }

    fn read(&self) -> Option<A>;

    fn scope(&self) -> &Head<Q, Self::Idx>;

    fn scope_mut(&mut self) -> &mut Head<Q, Self::Idx>;

    fn write(&mut self, symbol: A) -> Option<A>;

    fn position(&self) -> Self::Idx {
        self.scope().symbol
    }
}

pub trait Reader {
    type Output;

    fn read(&self) -> Self::Output;
}

pub trait WriteMut<T> {
    type Output;

    fn write(&mut self, data: T) -> Option<Self::Output>;
}
