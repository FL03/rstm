/*
    Appellation: symbol <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Symbol;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

pub struct InputAlpha<S = char>
where
    S: Symbol,
{
    pub symbols: Vec<S>,
}
