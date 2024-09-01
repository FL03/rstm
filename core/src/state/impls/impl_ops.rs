/*
    Appellation: impl_ops <state>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::State;

impl<Q> core::ops::Neg for State<Q>
where
    Q: core::ops::Neg,
{
    type Output = State<Q::Output>;

    fn neg(self) -> Self::Output {
        State(core::ops::Neg::neg(self.0))
    }
}

impl<'a, Q> core::ops::Neg for &'a State<Q>
where
    &'a Q: core::ops::Neg,
{
    type Output = State<<&'a Q as core::ops::Neg>::Output>;

    fn neg(self) -> Self::Output {
        State(core::ops::Neg::neg(self.get_ref()))
    }
}

impl<Q> core::ops::Not for State<Q>
where
    Q: core::ops::Not,
{
    type Output = State<Q::Output>;

    fn not(self) -> Self::Output {
        State(core::ops::Not::not(self.0))
    }
}

impl<'a, Q> core::ops::Not for &'a State<Q>
where
    &'a Q: core::ops::Not,
{
    type Output = State<<&'a Q as core::ops::Not>::Output>;

    fn not(self) -> Self::Output {
        State(core::ops::Not::not(self.get_ref()))
    }
}

impl<Q> num::traits::Num for State<Q> where Q: num::traits::Num {
    type FromStrRadixErr = Q::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Q::from_str_radix(str, radix).map(State)
    }
}

impl<Q> num::traits::One for State<Q>
where
    Q: PartialEq + num::traits::One,
{
    fn one() -> Self {
        State(Q::one())
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl<Q> num::traits::Zero for State<Q>
where
    Q: num::traits::Zero,
{
    fn zero() -> Self {
        State(Q::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

macro_rules! impl_assign_ops {
    (@impl $trait:ident::$call:ident) => {
        impl<Q: ::core::ops::$trait> ::core::ops::$trait for $crate::state::State<Q> {
            fn $call(&mut self, rhs: $crate::state::State<Q>) {
                ::core::ops::$trait::$call(self.get_mut(), rhs.get())
            }
        }

        impl<Q: ::core::ops::$trait> ::core::ops::$trait<Q> for $crate::state::State<Q> {
            fn $call(&mut self, rhs: Q) {
                ::core::ops::$trait::$call(self.get_mut(), rhs)
            }
        }
    };
    (alt: $($trait:ident::$call:ident),* $(,)?) => {
        paste::paste! {
            impl_assign_ops!($([<$trait Assign>]::[<$call _assign>]),*);
        }
    };
    ($($trait:ident::$call:ident),* $(,)?) => {
        $(
            impl_assign_ops!(@impl $trait::$call);
        )*
    };
}

macro_rules! impl_ops {
    (@impl $trait:ident::$call:ident) => {
        impl<Q> ::core::ops::$trait for State<Q>
        where
            Q: ::core::ops::$trait,
        {
            type Output = State<Q::Output>;

            fn $call(self, rhs: State<Q>) -> Self::Output {
                State(::core::ops::$trait::$call(self.get(), rhs.get()))
            }
        }

        impl<'a, Q> ::core::ops::$trait<&'a State<Q>> for State<Q>
        where
            Q: ::core::ops::$trait<&'a Q>,
        {
            type Output = State<Q::Output>;

            fn $call(self, rhs: &'a State<Q>) -> Self::Output {
                State(::core::ops::$trait::$call(self.get(), rhs.get_ref()))
            }
        }

        impl<'a, Q> ::core::ops::$trait<State<Q>> for &'a State<Q>
        where
            &'a Q: ::core::ops::$trait<Q>,
        {
            type Output = State<<&'a Q as ::core::ops::$trait<Q>>::Output>;

            fn $call(self, rhs: State<Q>) -> Self::Output {
                State(::core::ops::$trait::$call(self.get_ref(), rhs.get()))
            }
        }

        impl<'a, Q> ::core::ops::$trait<&'a State<Q>> for &'a State<Q>
        where
            &'a Q: ::core::ops::$trait<&'a Q>,
        {
            type Output = State<<&'a Q as ::core::ops::$trait<&'a Q>>::Output>;

            fn $call(self, rhs: &'a State<Q>) -> Self::Output {
                State(::core::ops::$trait::$call(self.get_ref(), rhs.get_ref()))
            }
        }
    };
    (@inner $trait:ident::$call:ident) => {
        impl<Q> ::core::ops::$trait<Q> for State<Q>
        where
            Q: ::core::ops::$trait,
        {
            type Output = State<Q::Output>;

            fn $call(self, rhs: Q) -> Self::Output {
                State(::core::ops::$trait::$call(self.get(), rhs))
            }
        }

        impl<'a, Q> ::core::ops::$trait<&'a Q> for State<Q>
        where
            Q: ::core::ops::$trait<&'a Q>,
        {
            type Output = State<Q::Output>;

            fn $call(self, rhs: &'a Q) -> Self::Output {
                State(::core::ops::$trait::$call(self.get(), rhs))
            }
        }

        impl<'a, Q> ::core::ops::$trait<Q> for State<&'a Q>
        where
            &'a Q: ::core::ops::$trait<Q>,
        {
            type Output = State<<&'a Q as ::core::ops::$trait<Q>>::Output>;

            fn $call(self, rhs: Q) -> Self::Output {
                State(::core::ops::$trait::$call(self.get(), rhs))
            }
        }

        impl<'a, Q> ::core::ops::$trait<Q> for &'a State<Q>
        where
            &'a Q: ::core::ops::$trait<Q>,
        {
            type Output = State<<&'a Q as ::core::ops::$trait<Q>>::Output>;

            fn $call(self, rhs: Q) -> Self::Output {
                State(::core::ops::$trait::$call(self.get_ref(), rhs))
            }
        }

        impl<'a, Q> ::core::ops::$trait<&'a Q> for &'a State<Q>
        where
            &'a Q: ::core::ops::$trait<&'a Q>,
        {
            type Output = State<<&'a Q as ::core::ops::$trait<&'a Q>>::Output>;

            fn $call(self, rhs: &'a Q) -> Self::Output {
                State(::core::ops::$trait::$call(self.get_ref(), rhs))
            }
        }
    };
    ($($trait:ident::$call:ident),* $(,)?) => {
        $(
            impl_ops!(@impl $trait::$call);
            impl_ops!(@inner $trait::$call);
            impl_assign_ops!(alt: $trait::$call);
        )*
    };
}

impl_ops! {
    Add::add,
    BitAnd::bitand,
    BitOr::bitor,
    BitXor::bitxor,
    Div::div,
    Mul::mul,
    Rem::rem,
    Shl::shl,
    Shr::shr,
    Sub::sub,
}
