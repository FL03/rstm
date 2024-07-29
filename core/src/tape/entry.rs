/*
    Appellation: entry <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]
use crate::Instruction;

pub struct Entry<'a, Q, S> {
    key: &'a mut usize,
    value: &'a mut Instruction<Q, S>,
}

impl<'a, Q, S> Entry<'a, Q, S> {
    pub fn new(key: &'a mut usize, value: &'a mut Instruction<Q, S>) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &usize {
        &self.key
    }

    pub fn set_key(&mut self, key: usize) {
        *self.key = key;
    }

    pub fn set_value(&mut self, value: Instruction<Q, S>) {
        *self.value = value;
    }
}
