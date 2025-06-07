/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! the state module implements the [`State`] type and its associated traits and types
#[doc(inline)]
pub use self::{error::*, halt::*, state::State};

pub(crate) mod state;

pub mod error;
pub mod halt;

mod impls {
    mod impl_ops;
    mod impl_repr;
    mod impl_state;
}
pub(crate) mod prelude {
    #[cfg(feature = "alloc")]
    pub use super::AnyState;
    #[doc(inline)]
    pub use super::MaybeState;

    #[doc(inline)]
    pub use super::error::*;
    #[doc(inline)]
    pub use super::halt::*;
    #[doc(inline)]
    pub use super::state::*;
}
#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "alloc")]
/// A type alias for a [State] whose inner value is the dynamically sized type of a boxed [`Any`](core::any::Any).
pub type AnyState = State<alloc::boxed::Box<dyn core::any::Any>>;
/// A type alias for a [State] whose inner value is a [core::mem::MaybeUninit] of generic type `Q`.
pub type MaybeState<Q = bool> = State<core::mem::MaybeUninit<Q>>;

/// [`RawState`] is a trait describing objects capable of being used as states in our library.
/// The trait contains a single associated trait, the context, or inner value of the state.
pub trait RawState: Send + Sync + core::fmt::Debug + core::fmt::Display {
    private!();
}

pub trait StdState: RawState
where
    Self: Clone + Default + PartialEq + PartialOrd,
{
}
/// The [`NumState`] trait extends the [`RawState`] trait to include numeric operations.
pub trait NumState: RawState
where
    Self: Copy
        + Eq
        + PartialOrd
        + core::ops::Add<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Rem<Output = Self>
        + core::ops::Neg<Output = Self>
        + core::ops::Not<Output = Self>
        + core::ops::AddAssign
        + core::ops::SubAssign
        + core::ops::MulAssign
        + core::ops::DivAssign
        + num_traits::One
        + num_traits::Zero
        + num_traits::ToPrimitive
        + num_traits::FromPrimitive,
{
}

/// The [`Stated`] trait defines the interface for stateful implementations in the library.
pub trait Stated {
    type Item;

    private!();

    fn get(self) -> Self::Item;

    fn get_ref(&self) -> &Self::Item;

    fn get_mut(&mut self) -> &mut Self::Item;

    fn set(&mut self, inner: Self::Item);
}

/*
 ************* Implementations *************
*/
macro_rules! impl_raw_state {
    ($($t:ty),* $(,)?) => {
        $(
            impl_raw_state!(@impl $t);
        )*
    };
    (@impl $t:ty) => {
        impl $crate::state::RawState for $t {
            seal!();
        }
    };
}

macro_rules! impl_state {
    (@impl $state:ident($($field:tt)*)) => {
        impl<Q> $crate::state::Stated for $state<Q> {
            type Item = Q;

            seal!();

            fn get(self) -> Q {
                self.$($field)*
            }

            fn get_ref(&self) -> &Q {
                &self.$($field)*
            }

            fn get_mut(&mut self) -> &mut Q {
                &mut self.$($field)*
            }

            fn set(&mut self, inner: Q) {
                self.$($field)* = inner;
            }
        }
    };
    ($($T:ident($($field:tt)*)),* $(,)?) => {
        $(
            impl_state!(@impl $T($($field)*));
        )*
    };
}

impl_raw_state! {
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f32, f64, bool, char,
}

#[cfg(feature = "alloc")]
impl_raw_state! {
    String,
}

impl_state! {
    Halt(0),
    State(0),
}

impl<'a, Q> RawState for &'a Q
where
    Q: 'a + RawState,
{
    seal!();
}

impl<'a, Q> RawState for &'a mut Q
where
    Q: 'a + RawState,
{
    seal!();
}
