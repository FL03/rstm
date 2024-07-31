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
///
/// Ideally, the alphabet should be implemented on unit enums since
/// each symbol can be represented as a unique variant and assigned
/// a particular value. The values of the variants may then be used
/// as pointers, specifiying the location of the symbol w.r.t. the
/// alphabet.
pub trait Alphabet {
    type Sym;

    fn len(&self) -> usize {
        self.to_vec().len()
    }

    fn to_vec(&self) -> Vec<Self::Sym>;
}

/// [Symbolic] is a trait denoting types that can be used as symbols;
/// this is useful for allowing symbols to represented with [char] or
/// be a position on the tape, value mapping for an alphabet,.
pub trait Symbolic
where
    Self: Clone
        + Copy
        + Eq
        + Ord
        + PartialEq
        + PartialOrd
        + core::fmt::Debug
        + core::fmt::Display
        + core::hash::Hash
        + Send
        + Sync
        + 'static
{
}

/*
 ************* Implementations *************
*/

impl<S: Symbolic> Alphabet for Vec<S> {
    type Sym = S;

    fn to_vec(&self) -> Vec<S> {
        self.clone()
    }
}

impl<S> Symbolic for S where
    S: Copy + Eq + Ord + core::fmt::Debug + core::fmt::Display + core::hash::Hash + Send + Sync + 'static
{
}
