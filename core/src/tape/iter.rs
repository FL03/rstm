/*
    Appellation: iter <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub struct Iter<'a, T> {
    tape: &'a T,
    index: usize,
}