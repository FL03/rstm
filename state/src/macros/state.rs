/*
    Appellation: state <module>
    Created At: 2025.08.30:00:11:24
    Contrib: @FL03
*/

/// the [`state!`] macro is a simple helper macro to create a [`State`](crate::state::State)
/// instance.
#[macro_export]
macro_rules! state {
    (@impl $state:expr) => {
        $crate::state::State($state)
    };
    ($($state:expr),* $(,)?) => {
        $(
            $crate::state!(@impl $state),
        )*
    };
}
