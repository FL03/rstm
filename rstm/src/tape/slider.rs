/*
    Appellation: slider <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![allow(dead_code)]
use crate::State;

pub struct Slider<Q, S> {
    state: *const State<Q>,
    symbol: *const S,
}
