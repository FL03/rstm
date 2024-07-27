/*
    Appellation: symbolic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// [Alphabet] abstractly describes the set of symbols used for both
/// the input and output of any given Turing machine.
///
/// ### Definition
///
/// An alphabet is formally defines to be a finite set of symbols.
pub trait Alphabet<S = char> {
    type Elem;

    fn symbols(&self) -> Vec<S>;
}

pub trait Symbol {
    fn as_ptr(&self) -> *const char;

    fn symbol(&self) -> char;
}

pub trait Symbolic:
    Clone + Eq + Ord + core::fmt::Debug + core::fmt::Display + core::hash::Hash
{
}

/*
 ************* Implementations *************
*/

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

impl<S> Symbolic for S where
    S: Clone + Eq + Ord + core::fmt::Debug + core::fmt::Display + core::hash::Hash
{
}
