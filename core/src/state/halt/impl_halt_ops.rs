/*
    appellation: impl_halt_ops <module>
    authors: @FL03
*/
use crate::state::Halt;

wrapper_impl_unary! {
    Halt(
        Neg::neg,
        Not::not,
    )
}

wrapper_impl_binary! {
    Halt(
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
