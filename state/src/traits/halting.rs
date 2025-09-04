/*
    appellation: halted <module>
    authors: @FL03
*/
use crate::traits::RawState;

/// The [`Halt`] trait establishes a common interface for any _haltable_ type;
pub trait Halt {
    private! {}

    fn is_halted(&self) -> bool;
}

pub trait HaltState: Halt + RawState {
    private! {}
}

/*
 ************* Implementations *************
*/
use crate::state::State;

impl<Q> HaltState for Q
where
    Q: Halt + RawState,
{
    seal! {}
}

impl<Q> Halt for State<Q>
where
    Q: RawState + Halt,
{
    seal!();

    fn is_halted(&self) -> bool {
        self.0.is_halted()
    }
}

impl<Q> Halt for &Q
where
    Q: Halt,
{
    seal!();

    fn is_halted(&self) -> bool {
        <Q as Halt>::is_halted(*self)
    }
}

impl<Q> Halt for &mut Q
where
    Q: Halt,
{
    seal!();

    fn is_halted(&self) -> bool {
        <Q as Halt>::is_halted(*self)
    }
}

impl<Q> Halt for Option<Q>
where
    Q: RawState,
{
    seal!();

    fn is_halted(&self) -> bool {
        self.is_none()
    }
}

impl<Q> Halt for Option<State<Q>>
where
    Q: RawState,
{
    seal!();

    fn is_halted(&self) -> bool {
        self.is_none()
    }
}

macro_rules! impl_num_haltable {
    (#[impl($trait:ident)] $($t:ty),* $(,)?) => {
        $(
            impl $trait for $t {
                seal!();

                fn is_halted(&self) -> bool {
                    *self == <$t>::MAX
                }
            }
        )*
    };
}

impl_num_haltable!(
    #[impl(Halt)]
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
);

impl Halt for f32 {
    seal!();

    fn is_halted(&self) -> bool {
        self.is_infinite() || self.is_nan()
    }
}

impl Halt for f64 {
    seal!();

    fn is_halted(&self) -> bool {
        self.is_infinite() || self.is_nan()
    }
}
