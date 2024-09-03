/*
    Appellation: hash_tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(feature = "std")]
use crate::Direction;
use std::collections::hash_map::{self, HashMap};

pub(crate) type Hdx = isize;

#[derive(Clone, Debug, Default)]
pub struct HashTape<V = char> {
    index: Hdx,
    store: HashMap<Hdx, V>,
    ticks: usize,
}

impl<V> HashTape<V> {
    pub fn new() -> HashTape<V> {
        HashTape {
            index: 0,
            store: HashMap::new(),
            ticks: 0,
        }
    }

    pub fn from_data(data: HashMap<Hdx, V>) -> HashTape<V> {
        HashTape {
            index: 0,
            store: data,
            ticks: 0,
        }
    }

    pub fn from_iter<I>(iter: I) -> HashTape<V>
    where
        I: IntoIterator<Item = (Hdx, V)>,
    {
        HashTape {
            index: 0,
            store: HashMap::from_iter(iter),
            ticks: 0,
        }
    }

    pub fn from_seq<I>(seq: I) -> HashTape<V>
    where
        I: IntoIterator<Item = V>,
    {
        let iter = seq.into_iter().enumerate().map(|(i, v)| (i as Hdx, v));
        Self::from_iter(iter)
    }
    /// Returns the current position of the head.
    pub fn current_position(&self) -> Hdx {
        self.index
    }
    /// Returns the total number of steps taken by the head.
    pub fn ticks(&self) -> usize {
        self.ticks
    }
    /// clears the tape.
    pub fn clear(&mut self) {
        self.index = 0;
        self.store.clear();
        self.ticks = 0;
    }
    /// Returns the entry in the tape at the current index.
    pub fn current_entry(&mut self) -> hash_map::Entry<Hdx, V> {
        self.store.entry(self.index)
    }
    /// Returns a mutable entry in the tape at the given index.
    pub fn entry(&mut self, index: Hdx) -> hash_map::Entry<Hdx, V> {
        self.store.entry(index)
    }

    /// Returns true if the tape contains the given index.
    pub fn contains_key(&self, index: Hdx) -> bool {
        self.store.contains_key(&index)
    }
    /// Returns true if the tape contains the given value.
    pub fn contains_value(&self, value: &V) -> bool
    where
        V: PartialEq,
    {
        self.values().any(|v| v == value)
    }
    /// Returns a reference to the value at the given index.
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
    /// Returns an immutable iterator over the tape.
    pub fn iter(&self) -> hash_map::Iter<Hdx, V> {
        self.store.iter()
    }
    /// Returns a mutable iterator over the tape.
    pub fn iter_mut(&mut self) -> hash_map::IterMut<Hdx, V> {
        self.store.iter_mut()
    }
    /// Returns an iterator over the keys of the tape.
    pub fn keys(&self) -> hash_map::Keys<Hdx, V> {
        self.store.keys()
    }
    /// Returns the number of elements in the tape.
    pub fn len(&self) -> usize {
        self.store.len()
    }
    /// Returns a mutable reference to the value of the head at the current position; on empty,
    /// the given value is inserted and returned.
    pub fn or_insert(&mut self, default: V) -> &mut V {
        self.store.entry(self.index).or_insert(default)
    }
    /// Returns a mutable reference to the value of the head at the current position; on empty,
    /// the function is evaluated and the result is inserted and returned.
    pub fn or_insert_with<F>(&mut self, default: F) -> &mut V
    where
        F: FnOnce() -> V,
    {
        self.store.entry(self.index).or_insert_with(default)
    }
    /// Returns a mutable reference to the value of the head at the current position; if the
    /// value is not present, the default value is inserted and returned.
    pub fn or_default(&mut self) -> &mut V
    where
        V: Default,
    {
        self.store.entry(self.index).or_default()
    }
    /// Removes the value at the given index.
    pub fn remove(&mut self, index: Hdx) -> Option<V> {
        self.store.remove(&index)
    }
    /// Returns an iterator over the values of the tape.
    pub fn values(&self) -> hash_map::Values<Hdx, V> {
        self.store.values()
    }

    /// Shifts the cursor in the given direction.
    pub fn shift(&mut self, direction: Direction) {
        self.index += direction;
        self.ticks += 1;
    }

    /// Returns a reference to the value at the current cursor position.
    pub fn read(&self) -> Option<&V> {
        self.store.get(&self.index)
    }

    pub fn read_mut(&mut self) -> &mut V
    where
        V: Default,
    {
        self.or_default()
    }

    pub fn write(&mut self, step: Direction, value: V) {
        let _ = self.store.insert(self.index, value);
        self.shift(step);
    }
}
