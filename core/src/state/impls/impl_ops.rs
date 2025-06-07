/*
    Appellation: impl_ops <state>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::{RawState, State};

impl<Q> core::ops::Neg for State<Q>
where
    Q: RawState + core::ops::Neg,
{
    type Output = State<Q::Output>;

    fn neg(self) -> Self::Output {
        State(core::ops::Neg::neg(self.0))
    }
}

impl<'a, Q> core::ops::Neg for &'a State<Q>
where
    &'a Q: RawState + core::ops::Neg,
{
    type Output = State<<&'a Q as core::ops::Neg>::Output>;

    fn neg(self) -> Self::Output {
        State(core::ops::Neg::neg(self.get()))
    }
}

impl<Q> core::ops::Not for State<Q>
where
    Q: RawState + core::ops::Not,
{
    type Output = State<Q::Output>;

    fn not(self) -> Self::Output {
        State(core::ops::Not::not(self.0))
    }
}

impl<'a, Q> core::ops::Not for &'a State<Q>
where
    &'a Q: RawState + core::ops::Not,
{
    type Output = State<<&'a Q as core::ops::Not>::Output>;

    fn not(self) -> Self::Output {
        State(core::ops::Not::not(self.get()))
    }
}

impl<Q> num_traits::Num for State<Q>
where
    Q: RawState + num_traits::Num,
{
    type FromStrRadixErr = Q::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Q::from_str_radix(str, radix).map(State)
    }
}

impl<Q> num_traits::One for State<Q>
where
    Q: RawState + PartialEq + num_traits::One,
{
    fn one() -> Self {
        State(Q::one())
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl<Q> num_traits::Zero for State<Q>
where
    Q: RawState + num_traits::Zero,
{
    fn zero() -> Self {
        State(Q::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

macro_rules! impl_bin_op {
    (@impl $wrap:ident($trait:ident::$method:ident)) => {
        impl<A, B, C> ::core::ops::$trait<$wrap<B>> for $wrap<A>
        where
            A: $crate::state::RawState + ::core::ops::$trait<B, Output = C>,
            B: $crate::state::RawState,
            C: $crate::state::RawState,
        {
            type Output = $wrap<C>;

            fn $method(self, rhs: $wrap<B>) -> Self::Output {
                $wrap(::core::ops::$trait::$method(self.0, rhs.0))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a $wrap<B>> for $wrap<A>
        where
            A: $crate::state::RawState + ::core::ops::$trait<&'a B, Output = C>,
            B: $crate::state::RawState,
            C: $crate::state::RawState,
        {
            type Output = $wrap<C>;

            fn $method(self, rhs: &'a $wrap<B>) -> Self::Output {
                $wrap(::core::ops::$trait::$method(self.value(), rhs.get()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<&'a $wrap<B>> for &'a $wrap<A>
        where
            &'a A: $crate::state::RawState + ::core::ops::$trait<&'a B, Output = C>,
            B: $crate::state::RawState,
            C: $crate::state::RawState,
        {
            type Output = $wrap<C>;

            fn $method(self, rhs: &'a $wrap<B>) -> Self::Output {
                $wrap(::core::ops::$trait::$method(self.get(), rhs.get()))
            }
        }

        impl<'a, A, B, C> ::core::ops::$trait<$wrap<B>> for &'a $wrap<A>
        where
            &'a A: $crate::state::RawState + ::core::ops::$trait<B, Output = C>,
            B: $crate::state::RawState,
            C: $crate::state::RawState,
        {
            type Output = $wrap<C>;

            fn $method(self, rhs: $wrap<B>) -> Self::Output {
                $wrap(::core::ops::$trait::$method(self.get(), rhs.value()))
            }
        }
    };

    ($wrap:ident($($trait:ident::$method:ident),* $(,)?)) => {
        $(impl_bin_op!(@impl $wrap($trait::$method));)*
    };
}

macro_rules! impl_assign_op {
    (@impl $wrap:ident($trait:ident::$method:ident)) => {
        impl<A, B> ::core::ops::$trait<B> for $wrap<A>
        where
            A: $crate::state::RawState + ::core::ops::$trait<B>,
            B: $crate::state::RawState,
        {
            fn $method(&mut self, rhs: B) {
                ::core::ops::$trait::$method(self.get_mut(), rhs)
            }
        }
    };

    ($wrap:ident($($trait:ident::$method:ident),* $(,)?)) => {
        $(impl_assign_op!(@impl $wrap($trait::$method));)*
    };
}

impl_assign_op! {
    State(
        AddAssign::add_assign,
        SubAssign::sub_assign,
        MulAssign::mul_assign,
        DivAssign::div_assign,
        RemAssign::rem_assign,
        BitAndAssign::bitand_assign,
        BitOrAssign::bitor_assign,
        BitXorAssign::bitxor_assign,
        ShlAssign::shl_assign,
        ShrAssign::shr_assign,
    )
}

impl_bin_op! {
    State(
        Add::add,
        Sub::sub,
        Mul::mul,
        Div::div,
        Rem::rem,
        BitAnd::bitand,
        BitOr::bitor,
        BitXor::bitxor,
        Shl::shl,
        Shr::shr,
    )
}
