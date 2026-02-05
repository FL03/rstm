/*
    Appellation: difference <module>
    Contrib: @FL03
*/
/// A binary operator defining the percent difference between two values where given input
/// is compared to the caller meaning the caller is said to be the _original_ value whilst
/// the input is said to be the _new_ value.
pub trait PercentChange<Rhs = Self> {
    type Output;

    /// Computes the percent difference between two values.
    fn percent_change(self, rhs: Rhs) -> Self::Output;
}

pub trait PercentDifference<Rhs = Self> {
    type Output;

    /// Computes the percent difference between two values.
    fn percent_difference(self, rhs: Rhs) -> Self::Output;
}

/*
 ************* Implementations *************
*/
// impl<A, B, C> PercentChange<B> for A
// where
//     C: core::ops::Div<A, Output = C>,
//     for<'b> B: core::ops::Sub<&'b A, Output = C>,
// {
//     type Output = C;

//     fn percent_change(self, rhs: B) -> Self::Output {
//         (rhs - &self) / self
//     }
// }

macro_rules! impl_percent {
    ($($T:ty),* $(,)?) => {
        $(impl_percent! { @impl $T })*
    };
    (@impl $T:ty) => {
        impl_percent! { @change $T }
        impl_percent! { @diff $T }
    };
    (@change $T:ty) => {
        impl PercentChange for $T {
            type Output = $T;

            fn percent_change(self, rhs: Self) -> Self::Output {
                let change = if self >= rhs {
                    self - rhs
                } else {
                    rhs - self
                };
                change / self
            }
        }
    };
    (@diff $T:ty) => {
        impl PercentDifference for $T {
            type Output = $T;

            fn percent_difference(self, rhs: Self) -> Self::Output {
                let change = if self >= rhs {
                    self - rhs
                } else {
                    rhs - self
                };
                (2 as $T) * change / (self + rhs)
            }
        }
    }
}

impl_percent! {
    f32, f64,
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_change() {
        let original = 50f32;
        let new = 75f32;
        let percent_change = original.percent_change(new);
        assert_eq! { percent_change, 0.5 } // 50% increase
    }

    #[test]
    fn test_percent_difference() {
        let (x, y) = (50f64, 75f64);
        assert_eq! { x.percent_difference(y), 0.4 };
        let (x, y) = (50u32, 75u32);
        assert_eq! { x.percent_difference(y), 4 / 10 };
    }
}
