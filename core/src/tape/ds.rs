/*
    Appellation: ds <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(unused)]
/// [DoubleTape] preserves the input, recording all actions taken on a seperate tape.
pub struct DoubleTape<I = char, O = char> {
    pub(crate) inputs: Vec<I>,
    pub(crate) output: Vec<O>,
}
