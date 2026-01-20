/*
    Appellation: increment <module>
    Contrib: @FL03
*/
/// [`Increment`] defines an unary operator for incrementing, or stepping up, a value by a
/// single unit.
pub trait Increment {
    fn inc(self) -> Self;
}
/// A mutable unary operator defining a self-incrementing operation
pub trait IncrementAssign {
    fn inc_assign(&mut self);
}
/// [`Decrement`] defines an unary operator for decrementing, or stepping down, a value by a
/// single unit.
pub trait Decrement {
    fn dec(self) -> Self;
}
/// A mutable unary operator defining a self-decrementing operation
pub trait DecAssign {
    fn dec_assign(&mut self);
}

/*
 ************* Implementations *************
*/
macro_rules! impl_inc_dec_for {
    (@impl $T:ty: $one:literal) => {
        impl Increment for $T {
            fn inc(self) -> Self {
                self + $one
            }
        }

        impl IncrementAssign for $T {
            fn inc_assign(&mut self) {
                *self += $one
            }
        }

        impl Decrement for $T {
            fn dec(self) -> Self {
                self - $one
            }
        }

        impl DecAssign for $T {
            fn dec_assign(&mut self) {
                *self -= $one
            }
        }
    };
    ($($one:literal {$($T:ty),* $(,)?});* $(;)?) => {
        $($(impl_inc_dec_for! { @impl $T: $one })*)*
    };
}

impl_inc_dec_for! {
    1 {
        u8, u16, u32, u64, u128, usize,
        i8, i16, i32, i64, i128, isize,
    };
    1.0 { f32, f64 };
}
