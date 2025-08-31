/*
    Appellation: tmh <module>
    Created At: 2025.08.31:00:16:37
    Contrib: @FL03
*/

mod impl_tmh;
mod impl_tmh_ext;

use rstm_core::head::Head;

/// The [`TMH`] is an implementation of a Turing Machine with a "moving head"; this behavior is
/// manifested here by using the current position of the head as its symbol, serving as a
/// mapping to a symbol on the tape. Every step taken by the machine will update the symbol of
/// the head, thus _moving_ it along the tape.
///
/// The implementation is one of the primary _drivers_ used by actors within the library.
/// By itself, the driver is not particularly useful, however, when given some input and a
/// program, it can be used to perform computations.
#[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd)]
#[repr(C)]
pub struct TMH<Q, A> {
    /// the head of the tape
    pub(crate) head: Head<Q, usize>,
    /// the memory of the actor
    pub(crate) tape: Vec<A>,
}
