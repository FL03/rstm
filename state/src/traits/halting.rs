/*
    appellation: halted <module>
    authors: @FL03
*/
use crate::traits::RawState;

/// The [`Halting`] trait defines an interface for any _haltable_ types.
pub trait Halting {
    private! {}

    fn is_halted(&self) -> bool;
}

pub trait HaltState: Halting + RawState {
    private! {}
}

/*
 ************* Implementations *************
*/
use crate::state::State;

impl<Q> HaltState for Q
where
    Q: Halting + RawState,
{
    seal! {}
}

impl<Q> Halting for State<Q>
where
    Q: RawState + Halting,
{
    seal!();

    fn is_halted(&self) -> bool {
        self.0.is_halted()
    }
}

impl<Q> Halting for &Q
where
    Q: Halting,
{
    seal!();

    fn is_halted(&self) -> bool {
        <Q as Halting>::is_halted(*self)
    }
}

impl<Q> Halting for &mut Q
where
    Q: Halting,
{
    seal!();

    fn is_halted(&self) -> bool {
        <Q as Halting>::is_halted(*self)
    }
}

impl<Q> Halting for Option<Q>
where
    Q: RawState,
{
    seal!();

    fn is_halted(&self) -> bool {
        self.is_none()
    }
}

impl<Q> Halting for Option<State<Q>>
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
    #[impl(Halting)]
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
);

impl Halting for f32 {
    seal!();

    fn is_halted(&self) -> bool {
        self.is_infinite() || self.is_nan()
    }
}

impl Halting for f64 {
    seal!();

    fn is_halted(&self) -> bool {
        self.is_infinite() || self.is_nan()
    }
}
