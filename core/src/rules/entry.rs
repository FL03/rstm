/*
    Appellation: entry <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use crate::rules::{StdHead, StdTail};

pub struct Entry<Q, S> {
    key: StdHead<Q, S>,
    value: StdTail<Q, S>,
}

impl<Q, S> Entry<Q, S> {
    pub fn new(key: StdHead<Q, S>, value: StdTail<Q, S>) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &StdHead<Q, S> {
        &self.key
    }

    pub fn value(&self) -> &StdTail<Q, S> {
        &self.value
    }
}
