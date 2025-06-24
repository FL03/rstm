/*
    appellation: stated <module>
    authors: @FL03
*/
use crate::state::RawState;
/// The [`Stated`] trait defines the interface for stateful implementations in the library.
pub trait Stated {
    /// the type of raw state
    type Item: RawState;

    private!();
    /// consumes the wrapper to return the inner state
    fn into_inner(self) -> Self::Item;
    /// returns a reference to the current state
    fn get(&self) -> &Self::Item;
    /// returns a mutable reference to the current state
    fn get_mut(&mut self) -> &mut Self::Item;
    /// update the current state and return a mutable reference to self
    fn set(&mut self, inner: Self::Item) -> &mut Self {
        *self.get_mut() = inner;
        self
    }
    /// [`replace`](core::mem::replace) the current state with another, returning the previous
    /// state
    fn replace(&mut self, inner: Self::Item) -> Self::Item {
        core::mem::replace(self.get_mut(), inner)
    }
    /// [`swap`](core::mem::swap) the current state with another
    fn swap(&mut self, other: &mut Self) {
        core::mem::swap(self.get_mut(), other.get_mut())
    }
    /// [`take`](core::mem::take) the current state, leaving the logical default in its place
    fn take(&mut self) -> Self::Item
    where
        Self::Item: Default,
    {
        core::mem::take(self.get_mut())
    }
}

macro_rules! stated {
    (@impl $state:ident($($field:tt)*)) => {
        impl<Q> $crate::state::Stated for $crate::state::$state<Q>
        where
            Q: $crate::state::RawState,
        {
            type Item = Q;

            seal!();

            fn into_inner(self) -> Q {
                self.$($field)*
            }

            fn get(&self) -> &Q {
                &self.$($field)*
            }

            fn get_mut(&mut self) -> &mut Q {
                &mut self.$($field)*
            }
        }
    };
    ($($T:ident($($field:tt)*)),* $(,)?) => {
        $(
            stated!(@impl $T($($field)*));
        )*
    };
}

stated! {
    State(0),
}
