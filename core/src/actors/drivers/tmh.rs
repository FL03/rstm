/*
    Appellation: tmh <module>
    Created At: 2025.08.31:00:16:37
    Contrib: @FL03
*/
use crate::rules::Head;

#[deprecated(
    since = "0.1.3",
    note = "The `TMH` driver is deprecated and will be removed; please use the `Head<Q, usize>` driver instead."
)]
/// The [`TMH`] implementation works to emulate the behaviors of a Turing machine with a
/// _moving head_ (TMH).
///
/// ## Usage
///
/// The [`TMH`] implementation works by maintaining a `head` of type [`Head<Q, usize>`],
/// allowing the current symbol to define the head's position on a tape. While the driver
/// contains its own tape, it is primarily used for handling any inputs into the system and
/// isn't actively updated during execution.  
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
    /// marker for the symbols
    pub(crate) _marker: core::marker::PhantomData<A>,
}
