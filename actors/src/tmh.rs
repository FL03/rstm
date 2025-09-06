/*
    Appellation: tmh <module>
    Created At: 2025.08.31:00:16:37
    Contrib: @FL03
*/

mod impl_tmh;
mod impl_tmh_ext;

use rstm_core::head::Head;

/// The [`TMH`] is an implementation of a Turing Machine with a "moving head". Here, we
/// manifest these behaviors by using the symbol of the head to represent its current position
/// along a sequential collection of symbols, a.k.a. the tape, which is represented as a [`Vec<A>`]
///
/// ## Usage
///
/// Before executing any particular program, the tape should be loaded up with the necessary
/// _inputs_ using the [`extend_tape`](TMH::extend_tape) method. The tape is represented
/// as a [`Vec<A>`], where `A` is the type of the symbols allowed on the tape. After setting
/// any inputs, we can use the [`execute`](TMH::execute) method to initialize a lazy executor
/// that will run the program whenever the `run`
///
#[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd)]
#[repr(C)]
pub struct TMH<Q, A> {
    /// the head of the tape
    pub(crate) head: Head<Q, usize>,
    /// the memory of the actor
    pub(crate) tape: Vec<A>,
}
