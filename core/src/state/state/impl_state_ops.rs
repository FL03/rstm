/*
    Appellation: impl_ops <state>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::{RawState, State};

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

wrapper_impl_unary! {
    State(
        Neg::neg,
        Not::not,
    )
}

wrapper_impl_binary! {
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
