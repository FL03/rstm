/*
    Appellation: entry <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use crate::rules::{Head, Tail};

pub struct Entry<Q, S> {
    key: Head<Q, S>,
    value: Tail<Q, S>,
}

impl<Q, S> Entry<Q, S> {
    pub fn new(key: Head<Q, S>, value: Tail<Q, S>) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &Head<Q, S> {
        &self.key
    }

    pub fn value(&self) -> &Tail<Q, S> {
        &self.value
    }
}
