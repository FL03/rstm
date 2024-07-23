/*
    Appellation: symbol <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[repr(transparent)]
pub struct Symbol<S = char>(pub S);


pub struct Alphabet<S = char> {
    pub symbols: Vec<Symbol<S>>,
}
