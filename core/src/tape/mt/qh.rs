/*
    Appellation: qh <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::State;

/// Irrespesctive of the type of head being discussed, the `actor` may either only know the
/// position or the symbol of the head.
pub union U<S> {
    position: usize, // ptr?,
    symbol: core::mem::ManuallyDrop<S>,
}

pub struct QH<Q, S> {
    pub(crate) state: State<Q>,
    pub(crate) sym: U<S>,
}
