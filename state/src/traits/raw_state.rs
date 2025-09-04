/*
    appellation: raw_state <module>
    authors: @FL03
*/
#[cfg(feature = "alloc")]
use alloc::string::String;

pub trait Stated {
    type Ctx;

    fn get(&self) -> &Self::Ctx;
    fn get_mut(&mut self) -> &mut Self::Ctx;
}

/// [`RawState`] is a trait describing objects capable of being used as states in our library.
/// The trait contains a single associated trait, the context, or inner value of the state.
pub trait RawState: Send + Sync + core::fmt::Debug {
    private!();
}
/// [`DisplayState`] is a trait that extends the [`RawState`] trait to include symbolic
/// operations and implementations.
pub trait DisplayState: RawState
where
    Self: core::fmt::Debug + core::fmt::Display,
{
    private!();
}
/// The [`HashState`] trait extends the [`RawState`] trait to include hashing capabilities,
/// streamlining the process of using states as keys in hash maps or sets.
pub trait HashState: RawState
where
    Self: Eq + core::hash::Hash,
{
    private!();
}
/// The [`StdState`] trait extends the base [`RawState`] trait to include additional traits
/// commonly used alongside the state.
pub trait StdState: RawState
where
    Self: Clone + Default + PartialEq + PartialOrd + core::fmt::Debug + core::fmt::Display,
{
    private!();
}
/// The [`NumState`] trait extends the [`RawState`] trait to include numeric operations.
pub trait NumState: StdState
where
    Self: Copy
        + Eq
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
    private!();
}

/*
 ************* Implementations *************
*/
impl<T> RawState for &T
where
    T: RawState,
{
    seal!();
}

impl<T> RawState for &mut T
where
    T: RawState,
{
    seal!();
}

impl<T> DisplayState for T
where
    T: RawState + core::fmt::Display,
{
    seal!();
}

impl<T> HashState for T
where
    T: RawState + Eq + core::hash::Hash,
{
    seal!();
}

impl<T> StdState for T
where
    T: RawState + Clone + Default + PartialEq + PartialOrd + core::fmt::Display,
{
    seal!();
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
        + core::ops::SubAssign
        + core::ops::MulAssign
        + core::ops::DivAssign
        + num_traits::One
        + num_traits::Zero
        + num_traits::ToPrimitive
        + num_traits::FromPrimitive,
{
    seal!();
}

impl<Q> RawState for Option<Q>
where
    Q: RawState,
{
    seal!();
}

impl<Q> RawState for core::ops::ControlFlow<Q, Q>
where
    Q: RawState,
{
    seal!();
}

impl<Q> RawState for core::mem::MaybeUninit<Q>
where
    Q: RawState,
{
    seal!();
}

#[cfg(feature = "alloc")]
impl RawState for Box<dyn RawState> {
    seal!();
}

macro_rules! impl_raw_state {
    ($($t:ty),* $(,)?) => {
        $(
            impl_raw_state!(@impl $t);
        )*
    };
    (@impl $t:ty) => {
        impl $crate::traits::RawState for $t {
            seal!();
        }
    };
}

impl_raw_state! {
    usize, u8, u16, u32, u64, u128,
    isize, i8, i16, i32, i64, i128,
    f32, f64, bool, char, str,
}

#[cfg(feature = "alloc")]
impl_raw_state! {
    String,
}
