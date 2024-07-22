/*
    Appellation: tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::{Direction, FsmError};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Tape<S = char> {
    pos: usize,
    store: Vec<S>,
    ticks: usize,
}

impl<S> Tape<S> {
    pub fn new() -> Self {
        Tape {
            pos: 0,
            store: Vec::<S>::new(),
            ticks: 0,
        }
    }
    /// Returns an owned reference to the store as a [slice](core::slice)
    pub fn as_slice(&self) -> &[S] {
        &self.store
    }
    /// Returns a mutable reference to the store as a [slice](core::slice)
    pub fn as_mut_slice(&mut self) -> &mut [S] {
        &mut self.store
    }
    /// Given an index, return a reference to the symbol at that index;
    /// panics if the index is out of bounds.
    pub fn get<I>(&self, idx: I) -> Option<&I::Output>
    where
        I: core::slice::SliceIndex<[S]>,
    {
        self.store.get(idx)
    }
    /// Returns the current position of the tape head;
    pub fn position(&self) -> usize {
        self.pos
    }
    /// Returns an owned reference to the current symbol on the tape
    pub fn read(&self) -> Result<&S, FsmError> {
        self.get(self.pos)
            .ok_or(FsmError::index_out_of_bounds(self.pos))
    }
    ///
    pub fn write(&mut self, symbol: S) {
        if self.pos < self.store.len() {
            self.store[self.pos] = symbol;
        } else {
            self.store.push(symbol);
        }
    }

    pub fn step(&mut self, direction: Direction) {
        self.pos = direction.apply(self.pos);
        self.ticks += 1;
    }

    pub fn step_left(&mut self) {
        self.step(Direction::Left);
    }

    pub fn step_right(&mut self) {
        self.step(Direction::Right);
    }

    pub fn print_tape(&self)
    where
        S: core::fmt::Display,
    {
        println!(
            "{}",
            self.store
                .iter()
                .enumerate()
                .map(|(i, c)| if i == self.pos {
                    format!("[{}]", &c)
                } else {
                    c.to_string()
                })
                .collect::<String>()
        );
    }
}

impl Tape {
    pub fn from_str(input: &str) -> Tape {
        Tape {
            pos: 0,
            store: input.chars().collect(),
            ticks: 0,
        }
    }
}

impl<T> core::fmt::Display for Tape<T>
where
    T: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{}",
            self.store
                .iter()
                .enumerate()
                .map(|(i, c)| if i == self.pos {
                    format!("[{}]", &c)
                } else {
                    c.to_string()
                })
                .collect::<String>()
        )
    }
}
