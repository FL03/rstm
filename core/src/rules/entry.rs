/*
    Appellation: entry <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use crate::{Head, Tail};

pub struct Entry<'a, Q, S> {
    key: &'a mut Head<Q, S>,
    value: &'a mut Tail<Q, S>,
}

impl<'a, Q, S> Entry<'a, Q, S> {
    pub fn new(key: &'a mut Head<Q, S>, value: &'a mut Tail<Q, S>) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &Head<Q, S> {
        &self.key
    }

    pub fn value(&self) -> &Tail<Q, S> {
        &self.value
    }
}
