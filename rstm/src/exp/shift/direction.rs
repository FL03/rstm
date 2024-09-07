/*
    Appellation: direction <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::prelude::Direction;

pub struct LinearShift<T: ?Sized> {
    pub(crate) direction: Direction,
    pub(crate) value: T,
}