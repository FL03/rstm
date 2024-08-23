/*
    Appellation: cursor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Direction;

/// Here, [Cursor] describes a type capable of tracking both the position and number of steps
/// taken by an actor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Cursor<Idx = usize> {
    /// The current position of the cursor.
    pub(crate) position: Idx,
    /// The number of steps taken by the cursor.
    pub(crate) ticks: usize,
}

impl<Idx> Cursor<Idx> {
    pub fn new() -> Cursor<Idx>
    where
        Idx: Default,
    {
        Cursor {
            position: Idx::default(),
            ticks: 0,
        }
    }
    /// Resets the cursor to its initial state.
    pub fn reset(&mut self)
    where
        Idx: Default,
    {
        self.position = Idx::default();
        self.ticks = 0;
    }
    /// Returns the current position of the cursor.
    pub const fn position(&self) -> &Idx {
        &self.position
    }

    pub fn ticks(&self) -> usize {
        self.ticks
    }

    pub fn set_position(&mut self, position: Idx) {
        self.position = position;
        self.on_update()
    }
    /// Shifts the cursor in the given direction.
    pub fn shift(&mut self, direction: Direction)
    where
        Idx: core::ops::AddAssign<Direction>,
    {
        self.position += direction;
        self.on_update()
    }

    pub fn shift_left(&mut self)
    where
        Idx: core::ops::AddAssign<Direction>,
    {
        self.shift(Direction::Left)
    }

    pub fn shift_right(&mut self)
    where
        Idx: core::ops::AddAssign<Direction>,
    {
        self.shift(Direction::Right)
    }

    pub fn stay(&mut self) {
        self.on_update()
    }

    pub(crate) fn on_update(&mut self) {
        self.ticks += 1;
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Ticker<T = usize> {
    pub(crate) ticks: T,
}

impl<T> Ticker<T> {
    pub fn new() -> Self
    where
        T: Default,
    {
        Self {
            ticks: T::default(),
        }
    }

    pub const fn get(&self) -> &T {
        &self.ticks
    }

    pub fn reset(&mut self)
    where
        T: Default,
    {
        self.ticks = T::default();
    }

    pub fn set(&mut self, ticks: T) {
        self.ticks = ticks;
    }

    pub fn tick(&mut self)
    where
        T: core::ops::AddAssign + num::One,
    {
        self.ticks += T::one();
    }
}

impl<T> core::convert::AsRef<T> for Ticker<T> {
    fn as_ref(&self) -> &T {
        &self.ticks
    }
}

impl<T> core::convert::AsMut<T> for Ticker<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.ticks
    }
}

impl<T> core::ops::Deref for Ticker<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.ticks
    }
}

impl<T> core::ops::DerefMut for Ticker<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ticks
    }
}

impl<T> core::iter::Iterator for Ticker<T>
where
    T: Copy + core::ops::AddAssign + num::One,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.tick();
        Some(self.ticks)
    }
}

impl<T> core::cmp::PartialEq<T> for Ticker<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.ticks == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor() {
        let mut cursor = Cursor::<isize>::new();
        assert_eq!(*cursor.position(), 0);
        assert_eq!(cursor.ticks(), 0);

        cursor.shift(Direction::Right);
        assert_eq!(*cursor.position(), 1);
        assert_eq!(cursor.ticks(), 1);

        cursor.shift(Direction::Left);
        assert_eq!(*cursor.position(), 0);
        assert_eq!(cursor.ticks(), 2);

        cursor.shift(Direction::Stay);
        assert_eq!(*cursor.position(), 0);
        assert_eq!(cursor.ticks(), 3);

        cursor.set_position(10);
        assert_eq!(*cursor.position(), 10);
        assert_eq!(cursor.ticks(), 4);
    }
}
