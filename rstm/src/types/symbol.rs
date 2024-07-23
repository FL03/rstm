/*
    Appellation: symbol <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::Symbol;

pub struct InputAlpha<S = char>
where
    S: Symbol,
{
    pub symbols: Vec<S>,
}
