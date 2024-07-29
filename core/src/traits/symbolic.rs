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
pub trait Alphabet: IntoIterator<Item = Self::Sym> {
    type Sym;

    fn len(&self) -> usize {
        self.to_vec().len()
    }

    fn to_vec(&self) -> Vec<Self::Sym>;
}

/// [Symbolic] is a generic trait automatically implemented for any
/// type that satisfies the following conditions:
/// - Clone
/// - Eq
/// - Ord
pub trait Symbolic
where
    Self: Clone
        + Eq
        + Ord
        + PartialEq
        + PartialOrd
        + core::fmt::Debug
        + core::fmt::Display
        + core::hash::Hash,
{
}

#[doc(hidden)]
pub trait Symbol: Symbolic {
    type Z;

    fn symbol(&self) -> char;

    fn is_symbol(&self, symbol: char) -> bool {
        self.symbol() == symbol
    }
    /// Returns the value assigned to the symbol;
    fn value(&self) -> Self::Z;
}

/*
 ************* Implementations *************
*/

impl Alphabet for Vec<char> {
    type Sym = char;

    fn to_vec(&self) -> Vec<char> {
        self.clone()
    }
}

impl<S> Symbolic for S where
    S: Clone + Eq + Ord + core::fmt::Debug + core::fmt::Display + core::hash::Hash
{
}
