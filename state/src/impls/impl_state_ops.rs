/*
    Appellation: impl_ops <state>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::State;
use crate::traits::RawState;
use num_traits::{Num, One, Zero};

impl<Q> Num for State<Q>
where
    Q: RawState + Num,
{
    type FromStrRadixErr = Q::FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Q::from_str_radix(str, radix).map(State)
    }
}

impl<Q> One for State<Q>
where
    Q: RawState + PartialEq + One,
{
    fn one() -> Self {
        State(Q::one())
    }

    fn is_one(&self) -> bool {
        self.0.is_one()
    }
}

impl<Q> Zero for State<Q>
where
    Q: RawState + Zero,
{
    fn zero() -> Self {
        State(Q::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

contained::impl_wrapper_unary! {
    State {
        Neg.neg,
        Not.not,
    }
}

contained::binary_wrapper! {
    impl State {
        Add.add,
        Sub.sub,
        Mul.mul,
        Div.div,
        Rem.rem,
        BitAnd.bitand,
        BitOr.bitor,
        BitXor.bitxor,
        Shl.shl,
        Shr.shr
    }
}
