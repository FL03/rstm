/*
    Appellation: hash_tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
// #![cfg(feature = "std")]
use crate::shift::Direction;
use std::collections::hash_map::{self, HashMap};

pub trait HashIndex: Eq + core::hash::Hash + core::ops::Neg {}

pub type Hdx = isize;

#[derive(Clone, Debug, Default)]
pub struct HashTape<V = char> {
    cursor: Hdx,
    store: HashMap<Hdx, V>,
    ticks: usize,
}

impl<V> HashTape<V> {
    pub fn new() -> HashTape<V> {
        HashTape {
            cursor: 0,
            store: HashMap::new(),
            ticks: 0,
        }
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
        self.store.clear();
        self.ticks = 0;
    }

    pub fn cursor(&self) -> Hdx {
        self.cursor
    }

    pub fn ticks(&self) -> usize {
        self.ticks
    }

    pub fn entry(&mut self, index: Hdx) -> hash_map::Entry<Hdx, V> {
        self.store.entry(index)
    }

    pub fn get(&self, index: Hdx) -> Option<&V> {
        self.store.get(&index)
    }
    /// Returns a mutable reference to the value at the given index.
    pub fn get_mut(&mut self, index: Hdx) -> Option<&mut V> {
        self.store.get_mut(&index)
    }
    /// Inserts a value at the given index.
    pub fn insert(&mut self, index: Hdx, value: V) {
        self.store.insert(index, value);
    }
    /// Returns true if the tape is empty.
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
    /// Returns the number of elements in the tape.
    pub fn len(&self) -> usize {
        self.store.len()
    }
    /// Removes the value at the given index.
    pub fn remove(&mut self, index: Hdx) -> Option<V> {
        self.store.remove(&index)
    }
    /// Shifts the cursor in the given direction.
    pub fn shift(&mut self, direction: Direction) {
        self.cursor += direction;
        self.ticks += 1;
    }
    /// Returns a mutable reference to the value of the head at the current position; on empty,
    /// the given value is inserted and returned.
    pub fn or_insert(&mut self, default: V) -> &mut V {
        self.store.entry(self.cursor).or_insert(default)
    }
    /// Returns a mutable reference to the value of the head at the current position; on empty,
    /// the function is evaluated and the result is inserted and returned.
    pub fn or_insert_with<F>(&mut self, default: F) -> &mut V
    where
        F: FnOnce() -> V,
    {
        self.store.entry(self.cursor).or_insert_with(default)
    }
    /// Returns a mutable reference to the value of the head at the current position; if the
    /// value is not present, the default value is inserted and returned.
    pub fn or_default(&mut self) -> &mut V
    where
        V: Default,
    {
        self.store.entry(self.cursor).or_default()
    }
    /// Returns a reference to the value at the current cursor position.
    pub fn read(&self) -> Option<&V> {
        self.store.get(&self.cursor)
    }

    pub fn write(&mut self, value: V) {
        let _ = self.store.insert(self.cursor, value);
    }
}
