/*
    Appellation: model <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Alphabet;

#[doc(hidden)]
pub trait Turing<Q> {
    type Alpha: Alphabet; // input alphabet
    type Beta: Alphabet; // output alphabet

    fn step(&self, input: Self::Alpha) -> Self::Beta;
}
