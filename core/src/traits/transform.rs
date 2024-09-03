/*
    Appellation: transform <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Direction, Head, State};

/// [Handle] is a generic trait describing objects capable of handling some input and producing
/// some output.
pub trait Handle<T> {
    type Output;

    fn handle(&mut self, args: T) -> Self::Output;
}

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

pub trait Driver<Q, A> {
    type Head;

    fn read(&self) -> Option<A>;

    fn scope(&self) -> Head<&'_ Q, &'_ A>;

    fn write(&mut self, symbol: A) -> Option<A>;
}

pub trait Read {
    type Output;

    fn read(&self) -> Self::Output;
}

pub trait Write {
    type Output;

    fn write(&self) -> Self::Output;
}
