/*
    Appellation: cursor <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [Cursor] contains contextual information about the position of the head of a tape.
///
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Cursor<T = char> {
    pub(crate) pos: usize,
    pub(crate) ptr: *mut T,
    pub(crate) step: usize,
}

impl<T> Cursor<T> {
    pub fn new(ptr: *mut T) -> Self {
        Self {
            pos: 0,
            ptr,
            step: 0,
        }
    }

    pub fn from_vec(vec: &mut Vec<T>) -> Self {
        Self::new(vec.as_mut_ptr())
    }

    pub fn pointer(&self) -> *mut T {
        self.ptr
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn step(&self) -> usize {
        self.step
    }

    pub fn update(&mut self, cursor: usize) {
        self.pos = cursor;
        self.next();
    }

    fn next(&mut self) {
        self.step += 1;
    }
}
