/*
    Appellation: actor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::Head;
use crate::state::State;

pub struct Actor<T> {
    pub(crate) head: Head<T>,
}

impl<T> Actor<T> {
    pub fn new(head: Head<T>) -> Self {
        Self { head }
    }

    pub const fn head(&self) -> &Head<T> {
        &self.head
    }

    pub fn head_mut(&mut self) -> &mut Head<T> {
        &mut self.head
    }
    /// Returns the current [state](State) the agent is in
    pub fn current_state(&self) -> State<&'_ T> {
        self.head.state()
    }
}
