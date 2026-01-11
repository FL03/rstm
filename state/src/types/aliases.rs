/*
    Appellation: aliases <module>
    Created At: 2026.01.11:11:18:54
    Contrib: @FL03
*/
use crate::state::{Halter, State};

#[cfg(feature = "alloc")]
/// A type alias for a [State] whose inner value is the dynamically sized type of a boxed [`Any`](core::any::Any).
pub type AnyState = State<alloc::boxed::Box<dyn core::any::Any>>;
/// A type alias for a [`State`] instance configured to leverage the [`ControlFlow`](core::ops::ControlFlow)
/// enum as its inner type. This is a useful shortcut for representing a halt state.
pub type ControlState<Q = usize> = State<core::ops::ControlFlow<Q, Q>>;
/// A type alias for a [`State`] equipped with the dedicated [`Halt`] enum as its inner type.
pub type HaltState<Q = u8> = State<Halter<Q>>;
/// A [`State`] alias allowing potentially uninitialized inner values using the [`MaybeUninit`](core::mem::MaybeUninit)
/// implementation.
pub type MaybeState<Q = bool> = State<core::mem::MaybeUninit<Q>>;
/// A state whose halting value is defined by the [`None`](Option::None) variant.
pub type OptionState<Q> = State<Option<Q>>;
