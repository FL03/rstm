/*
    Appellation: symbolic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Alphabet<S = char> {
    type Elem;

    fn symbols(&self) -> Vec<S>;
}

pub trait AsRefChar {
    fn as_char(&self) -> &char;
}


pub trait Symbol {
    fn as_ptr(&self) -> *const char;

    fn symbol(&self) -> char;
}

pub trait Symbolic: Eq + Ord + core::fmt::Debug + core::fmt::Display + core::hash::Hash {}

/*
 ************* Implementations *************
*/

impl<T> AsRefChar for T where T: AsRef<char> {
    fn as_char(&self) -> &char {
        self.as_ref()
    }
}

impl Alphabet for Vec<char> {
    type Elem = char;

    fn symbols(&self) -> Vec<char> {
        self.clone()
    }
}

impl Symbol for char {
    fn as_ptr(&self) -> *const char {
        self as *const char
    }

    fn symbol(&self) -> char {
        *self
    }
}

impl<S> Symbolic for S where S: Symbol + Eq + Ord + core::fmt::Debug + core::fmt::Display + core::hash::Hash {}