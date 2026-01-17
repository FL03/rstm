/*
    Appellation: tmh <module>
    Created At: 2025.08.31:00:16:37
    Contrib: @FL03
*/
use crate::rules::Head;

/// The [`TMH`] is an implementation of a Turing Machine with a "moving head". Here, we
/// manifest these behaviors by using the symbol of the head to represent its current position
/// along a sequential collection of symbols, a.k.a. the tape, which is represented as a [`Vec<A>`]
///
/// ## Usage
///
/// The [`TMH`] implementation works by maintaining a `head` of type [`Head<Q, usize>`],
/// allowing the current symbol to define the head's position on a tape. While the driver
/// contains its own tape, it is recommended to use an external tape to capture machine
/// outputs. This follows the formal definition of a Turing machine where there is both an
/// input and an output tape.
///
/// Before executing any particular program, the tape should be loaded up with the necessary
/// _inputs_ using the [`extend_tape`](TMH::extend_tape) method. The tape is represented
/// as a [`Vec<A>`], where `A` is the type of the symbols allowed on the tape. After setting
/// any inputs, we can use the [`load`](TMH::load) method to initialize a lazy executor
/// that will run the program whenever the `run`
///
#[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd)]
#[repr(C)]
pub struct TMH<Q, A> {
    /// the head of the tape
    pub(crate) head: Head<Q, usize>,
    /// the input tape of the Turing machine
    pub(crate) input: Vec<A>,
}
