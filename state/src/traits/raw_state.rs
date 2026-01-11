/*
    appellation: raw_state <module>
    authors: @FL03
*/
#[cfg(feature = "alloc")]
use alloc::string::String;

/// [`RawState`] is a sealed marker trait used to define objects used to represent states.
/// The primary benefit of such a trait is preventing cases where the [`State`](crate::State)
/// implementation is used as a state itself: `State<State<Q>>`.
pub trait RawState {
    private! {}
}
/// The [`HashState`] trait extends the [`RawState`] trait to include hashing capabilities,
/// streamlining the process of using states as keys in hash maps or sets.
pub trait HashState: RawState
where
    Self: Eq + core::hash::Hash,
{
    private! {}
}
/// The [`StdState`] trait extends the [`RawState`] trait to include standard traits commonly
/// used for state manipulation and comparison.
pub trait StdState
where
    Self: Clone + Default + PartialEq + PartialOrd + core::fmt::Debug + core::fmt::Display,
{
    private! {}
}
/// The [`NumState`] trait extends the [`RawState`] trait to include numeric operations.
pub trait NumState: StdState
where
    Self: Copy
        + Default
        + Eq
        + PartialOrd
        + core::ops::Add<Output = Self>
        + core::ops::Div<Output = Self>
        + core::ops::Mul<Output = Self>
        + core::ops::Neg<Output = Self>
        + core::ops::Not<Output = Self>
        + core::ops::Rem<Output = Self>
        + core::ops::Sub<Output = Self>
        + core::ops::AddAssign
        + core::ops::DivAssign
        + core::ops::MulAssign
        + core::ops::RemAssign
        + core::ops::SubAssign
        + num_traits::One
        + num_traits::Zero
        + num_traits::ToPrimitive
        + num_traits::FromPrimitive,
{
    private! {}
}

/*
 ************* Implementations *************
*/
impl RawState for () {
    seal! {}
}

impl<T> RawState for &T
where
    T: RawState,
{
    seal! {}
}

impl<T> RawState for &mut T
where
    T: RawState,
{
    seal! {}
}

impl<T> HashState for T
where
    T: RawState + Eq + core::hash::Hash,
{
    seal! {}
}

impl<T> StdState for T
where
    T: RawState + Clone + Default + PartialEq + PartialOrd + core::fmt::Debug + core::fmt::Display,
{
    seal! {}
}

impl<T> NumState for T
where
    T: StdState
        + Copy
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
        + core::ops::DivAssign
        + core::ops::MulAssign
        + core::ops::RemAssign
        + core::ops::SubAssign
        + num_traits::One
        + num_traits::Zero
        + num_traits::ToPrimitive
        + num_traits::FromPrimitive,
{
    seal! {}
}

impl<Q1, Q2> RawState for core::ops::ControlFlow<Q1, Q2>
where
    Q1: RawState,
    Q2: RawState,
{
    seal! {}
}

impl<Q> RawState for core::mem::MaybeUninit<Q>
where
    Q: RawState,
{
    seal! {}
}

#[cfg(feature = "alloc")]
impl RawState for Box<dyn RawState> {
    seal! {}
}

macro_rules! impl_raw_state {
    ($($t:ty),* $(,)?) => {
        $(impl_raw_state!{ @impl $t })*
    };
    ($($($cont:ident)::*<$T:ident $(where $($rest:tt)*)?>),* $(,)?) => {
        $(impl_raw_state! {
            @impl<$T> $cont::* $(where $($rest)*)?
        })*
    };
    (@impl $t:ty) => {
        impl $crate::traits::RawState for $t {
            seal! {}
        }
    };
    (@impl $($cont:ident)::*$(<$($T:ident),*>)? $(where $($rest:tt)*)?) => {
        impl<$($T),*> $crate::traits::RawState for $(cont)::*$(<$($T),*>)? $(where $($rest)*)? {
            seal! {}
        }
    };
}

impl_raw_state! {
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f32, f64,
    bool, char, str
}

#[cfg(feature = "alloc")]
impl_raw_state! {
    String,
}
