/*
    Appellation: hash_tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Direction;
use std::collections::hash_map::{self, HashMap};

pub(crate) type Hdx = isize;

#[derive(Clone, Debug, Default)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(default, rename_all = "snake_case")
)]
pub struct HashTape<V = char> {
    pub(crate) index: Hdx,
    pub(crate) store: HashMap<Hdx, V>,
    pub(crate) ticks: usize,
}

impl<V> HashTape<V> {
    /// returns a new empty tape.
    pub fn new() -> HashTape<V> {
        HashTape {
            index: 0,
            store: HashMap::new(),
            ticks: 0,
        }
    }
    /// returns a new empty tape with the given capacity.
    pub fn with_capacity(capacity: usize) -> HashTape<V> {
        let store = HashMap::with_capacity(capacity);
        HashTape::from_data(store)
    }
    /// returns a new tape from the given data.
    pub const fn from_data(data: HashMap<Hdx, V>) -> HashTape<V> {
        HashTape {
            index: 0,
            store: data,
            ticks: 0,
        }
    }
    /// constructs a new tape from an iterator.
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
    /// constructs a new tape from a sequence of values, using the index as the key.
    pub fn from_seq<I>(seq: I) -> HashTape<V>
    where
        I: IntoIterator<Item = V>,
    {
        let iter = seq.into_iter().enumerate().map(|(i, v)| (i as Hdx, v));
        Self::from_iter(iter)
    }
    /// returns the current position of the head.
    pub const fn current_index(&self) -> Hdx {
        self.index
    }
    /// returns the total number of steps taken by the head.
    pub const fn ticks(&self) -> usize {
        self.ticks
    }
    /// returns a reference to the store.
    pub const fn store(&self) -> &HashMap<Hdx, V> {
        &self.store
    }
    /// returns a mutable reference to the store.
    pub const fn store_mut(&mut self) -> &mut HashMap<Hdx, V> {
        &mut self.store
    }
    /// update the current index and return a mutable reference to the tape
    pub fn set_index(&mut self, index: Hdx) -> &mut Self {
        self.index = index;
        self
    }
    /// overwrites the current store and returns a mutable reference to the tape.
    pub fn set_store(&mut self, store: HashMap<Hdx, V>) -> &mut Self {
        self.store = store;
        self
    }
    /// consumes the current instance to create another with the given index
    pub fn with_index(self, index: Hdx) -> Self {
        HashTape { index, ..self }
    }
    /// consumes the current instance to create another with the given store
    pub fn with_store(self, store: HashMap<Hdx, V>) -> Self {
        HashTape { store, ..self }
    }
    /// [`replace`](core::mem::replace) the current tick count with the given value and return
    /// the previous value.
    pub(crate) const fn replace_ticks(&mut self, ticks: usize) -> usize {
        core::mem::replace(&mut self.ticks, ticks)
    }
    /// clears the tape.
    pub fn clear(&mut self) {
        self.index = 0;
        self.store_mut().clear();
        self.ticks = 0;
    }
    /// Returns a mutable entry in the tape at the given index.
    pub fn entry(&mut self, index: Hdx) -> hash_map::Entry<'_, Hdx, V> {
        self.store_mut().entry(index)
    }
    /// Returns true if the tape contains the given index.
    pub fn contains_key(&self, index: Hdx) -> bool {
        self.store().contains_key(&index)
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
        self.store().get(&index)
    }
    /// Returns a mutable reference to the value at the given index.
    pub fn get_mut(&mut self, index: Hdx) -> Option<&mut V> {
        self.store_mut().get_mut(&index)
    }
    /// Inserts a value at the given index.
    pub fn insert(&mut self, index: Hdx, value: V) {
        self.store_mut().insert(index, value);
    }
    /// Returns true if the tape is empty.
    pub fn is_empty(&self) -> bool {
        self.store().is_empty()
    }
    /// Returns the number of elements in the tape.
    pub fn len(&self) -> usize {
        self.store().len()
    }
    /// Returns a mutable reference to the value of the head at the current position; on empty,
    /// the given value is inserted and returned.
    pub fn or_insert(&mut self, default: V) -> &mut V {
        self.read().or_insert(default)
    }
    /// Returns a mutable reference to the value of the head at the current position; on empty,
    /// the function is evaluated and the result is inserted and returned.
    pub fn or_insert_with<F>(&mut self, default: F) -> &mut V
    where
        F: FnOnce() -> V,
    {
        self.read().or_insert_with(default)
    }
    /// Returns a mutable reference to the value of the head at the current position; if the
    /// value is not present, the default value is inserted and returned.
    pub fn or_default(&mut self) -> &mut V
    where
        V: Default,
    {
        self.read().or_default()
    }
    /// Removes the value at the given index.
    pub fn remove(&mut self, index: Hdx) -> Option<V> {
        self.store_mut().remove(&index)
    }
    /// Returns an immutable iterator over the tape.
    pub fn iter(&self) -> hash_map::Iter<'_, Hdx, V> {
        self.store().iter()
    }
    /// Returns a mutable iterator over the tape.
    pub fn iter_mut(&mut self) -> hash_map::IterMut<'_, Hdx, V> {
        self.store_mut().iter_mut()
    }
    /// Returns an iterator over the keys of the tape.
    pub fn keys(&self) -> hash_map::Keys<'_, Hdx, V> {
        self.store().keys()
    }
    /// Returns an iterator over the values of the tape.
    pub fn values(&self) -> hash_map::Values<'_, Hdx, V> {
        self.store().values()
    }
    /// Shifts the cursor in the given direction.
    pub fn shift(&mut self, direction: Direction) {
        self.index += direction;
        self.tick();
    }
    /// increments the tick count by one, returns the previous tick count.
    pub const fn tick(&mut self) -> usize {
        self.replace_ticks(self.ticks() + 1)
    }
    /// returns the [`Entry`](hash_map::Entry) for the current index
    pub fn read(&mut self) -> hash_map::Entry<'_, Hdx, V> {
        self.entry(self.index)
    }
    /// write the
    pub fn write(&mut self, step: Direction, value: V) -> crate::Result<()> {
        // read the tape and write the value
        self.read().insert_entry(value);
        // shift the tape head in the given direction
        self.shift(step);
        Ok(())
    }
}
