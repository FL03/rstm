/*
    Appellation: shift <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Shift<T> {
    type Output;

    fn shift(&self, tape: &T) -> Self::Output;
}
