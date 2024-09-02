/*
    Appellation: tail <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::builder::TailBuilder;

use crate::{Direction, Head, State};

/// The [Tail] is a 3-tuple containing the direction, state, and symbol that an actor is
/// instructed to execute whenever it assumes the head configuration assigned to the tail.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
pub struct Tail<Q = String, S = char> {
    pub direction: Direction,
    #[cfg_attr(feature = "serde", serde(alias = "next_state"))]
    pub state: State<Q>,
    #[cfg_attr(
        feature = "serde",
        serde(alias = "next_symbol", alias = "write_symbol")
    )]
    pub symbol: S,
}

impl<Q, S> Tail<Q, S> {
    pub fn new(direction: Direction, State(state): State<Q>, symbol: S) -> Self {
        Self {
            direction,
            state: State(state),
            symbol,
        }
    }

    pub fn create() -> TailBuilder<Q, S> {
        TailBuilder::new(Direction::Right)
    }
    /// Returns the direction, state, and symbol as a 3-tuple
    pub fn as_tuple(&self) -> (Direction, &State<Q>, &S) {
        (self.direction, &self.state, &self.symbol)
    }
    /// Consumes the tail and returns the direction, state, and symbol as a 3-tuple
    pub fn into_tuple(self) -> (Direction, State<Q>, S) {
        (self.direction, self.state, self.symbol)
    }
    /// Returns the direction the [head](StdHead) is instructed to move
    pub fn direction(&self) -> Direction {
        self.direction
    }
    /// Returns the next state with an immutable reference to the inner value
    pub fn state(&self) -> State<&'_ Q> {
        self.state.to_ref()
    }
    /// Returns the next state with a mutable reference to the inner value
    pub fn state_mut(&mut self) -> State<&'_ mut Q> {
        self.state.to_mut()
    }
    /// Returns the symbol the [head](Head) is instructed to write
    pub const fn symbol(&self) -> &S {
        &self.symbol
    }
    /// Consumes the tail and returns a new instance of the [Head]
    pub fn into_head(self) -> Head<Q, S> {
        Head {
            state: self.state,
            symbol: self.symbol,
        }
    }
    /// Returns an instance of the [head](Head) where each element within
    /// the created instance is an immutable reference
    pub fn as_head(&self) -> Head<&Q, &S> {
        Head {
            state: self.state.to_ref(),
            symbol: &self.symbol,
        }
    }

    pub fn to_ref(&self) -> Tail<&'_ Q, &'_ S> {
        Tail {
            direction: self.direction,
            state: self.state.to_ref(),
            symbol: &self.symbol,
        }
    }

    pub fn to_mut(&mut self) -> Tail<&'_ mut Q, &'_ mut S> {
        Tail {
            direction: self.direction,
            state: self.state.to_mut(),
            symbol: &mut self.symbol,
        }
    }

    pub fn into_owned(self) -> Tail<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Tail {
            direction: self.direction,
            state: self.state,
            symbol: self.symbol,
        }
    }

    pub fn to_owned(&self) -> Tail<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Tail {
            direction: self.direction,
            state: self.state.to_owned(),
            symbol: self.symbol.clone(),
        }
    }
}

impl<'a, Q, S> Tail<&'a Q, &'a S> {
    pub fn cloned(&self) -> Tail<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Tail {
            direction: self.direction,
            state: self.state.cloned(),
            symbol: self.symbol.clone(),
        }
    }

    pub fn copied(&self) -> Tail<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Tail {
            direction: self.direction,
            state: self.state.copied(),
            symbol: *self.symbol,
        }
    }
}

impl<'a, Q, S> Tail<&'a mut Q, &'a mut S> {
    pub fn cloned(&self) -> Tail<Q, S>
    where
        Q: Clone,
        S: Clone,
    {
        Tail {
            direction: self.direction,
            state: self.state.cloned(),
            symbol: self.symbol.clone(),
        }
    }

    pub fn copied(&self) -> Tail<Q, S>
    where
        Q: Copy,
        S: Copy,
    {
        Tail {
            direction: self.direction,
            state: self.state.copied(),
            symbol: *self.symbol,
        }
    }
}

impl<Q, S> core::fmt::Debug for Tail<Q, S>
where
    Q: core::fmt::Debug,
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Tail")
            .field(&self.direction)
            .field(&self.state)
            .field(&self.symbol)
            .finish()
    }
}

impl<Q, S> core::fmt::Display for Tail<Q, S>
where
    Q: core::fmt::Display,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}({}, {})", self.direction, self.state, self.symbol)
    }
}

mod builder {
    use super::*;

    pub struct TailBuilder<Q, S> {
        direction: Direction,
        state: Option<State<Q>>,
        symbol: Option<S>,
    }

    impl<Q, S> TailBuilder<Q, S> {
        pub fn new(direction: Direction) -> Self {
            Self {
                direction,
                state: None,
                symbol: None,
            }
        }
        /// Sets the direction
        pub fn direction(self, direction: Direction) -> Self {
            Self { direction, ..self }
        }
        /// Sets the next [state](State)
        pub fn state(self, State(state): State<Q>) -> Self {
            Self {
                state: Some(State(state)),
                ..self
            }
        }
        /// Sets the symbol
        pub fn symbol(self, symbol: S) -> Self {
            Self {
                symbol: Some(symbol),
                ..self
            }
        }
        /// Consumes the builder and returns a new instance of the [tail](Tail)
        pub fn build(self) -> Result<Tail<Q, S>, crate::Error> {
            if self.state.is_none() {
                return Err("Missing state".into());
            }

            if self.symbol.is_none() {
                return Err("Missing symbol".into());
            }
            Ok(Tail::new(
                self.direction,
                self.state.expect("State is required"),
                self.symbol.expect("Symbol is required"),
            ))
        }
    }
}
