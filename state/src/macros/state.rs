/*
    Appellation: state <module>
    Created At: 2025.08.30:00:11:24
    Contrib: @FL03
*/
#![cfg(feature = "macros")]

/// the [`s!`] macro is a simple helper macro to create a [`State`](crate::state::State)
/// instance.
#[macro_export]
macro_rules! s {
    ($($state:expr),* $(,)?) => {
        $(
            $crate::state::State($state),
        )*
    };
}
