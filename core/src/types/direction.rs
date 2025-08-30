/*
    Appellation: direction <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// [Direction] enumerates the various directions a head can move, namely: left, right, and stay.
///
/// The included methods and implementations aim to streamline the conversion between [Direction] and other types.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    variants::VariantConstructors,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
#[strum(serialize_all = "lowercase")]
pub enum Direction {
    /// Represents a single left shift
    #[cfg_attr(
        feature = "serde",
        serde(
            alias = "left",
            alias = "l",
            alias = "L",
            alias = "LEFT",
            alias = "Left"
        )
    )]
    Left = -1,
    #[cfg_attr(
        feature = "serde",
        serde(
            alias = "right",
            alias = "r",
            alias = "R",
            alias = "RIGHT",
            alias = "Right"
        )
    )]
    /// Represents a single right shift
    Right = 1,
    #[cfg_attr(
        feature = "serde",
        serde(
            alias = "stay",
            alias = "s",
            alias = "S",
            alias = "STAY",
            alias = "Stay"
        )
    )]
    /// Represents no movement
    Stay = 0,
}

/*
 ************* Implementations *************
*/
impl Direction {
    /// Converts an [i8] value into a [`Direction`] by taking the modulus of the value.
    /// The function uses a modulator of 2 to determine the direction since there are
    /// only two actionable directions ([left](Direction::Left) and [right](Direction::Right)).
    pub fn from_i8(value: i8) -> Self {
        match value % 2 {
            -1 => Self::Left,
            1 => Self::Right,
            _ => Self::Stay,
        }
    }
    /// Converts a [char] value into a direction; matches the value to the corresponding
    /// [direction](Direction).
    pub fn from_char(value: char) -> Self {
        match value {
            'L' | 'l' => Self::Left,
            'R' | 'r' => Self::Right,
            _ => Self::Stay,
        }
    }
    #[allow(clippy::should_implement_trait)]
    /// Converts a [str] value into a [Direction] by matching the value to the corresponding
    /// variant; defaults to [`Stay`](Direction::Stay) if the value does not match accepted
    /// representations of neither [left](Direction::Left) nor [right](Direction::Right).
    pub fn from_str(value: &str) -> Self {
        match value {
            "left" | "Left" | "LEFT" | "l" | "L" => Self::Left,
            "right" | "Right" | "RIGHT" | "r" | "R" => Self::Right,
            _ => Self::Stay,
        }
    }
    /// Returns a [char] representation of the [direction](Direction).
    ///
    /// ### standard [char] representation
    ///
    /// 'L' => [Direction::Left]
    /// 'R' => [Direction::Right]
    /// 'S' => [Direction::Stay]
    pub fn as_char(&self) -> char {
        match self {
            Self::Left => 'L',
            Self::Right => 'R',
            Self::Stay => 'S',
        }
    }
    /// Returns a [str] representation of the [direction](Direction).
    pub fn as_str(&self) -> &str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Stay => "stay",
        }
    }

    /// Applies the shift to the given position in the [direction](Direction) specified by the
    /// current instance. This is done using the [`wrapping_add_signed`](usize::wrapping_add_signed)
    /// method.
    pub fn apply_unsigned(self, cur: usize) -> usize {
        cur.wrapping_add_signed(self as isize)
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::Stay
    }
}

impl<T> core::ops::Add<T> for Direction
where
    T: core::ops::Add<Output = T> + core::ops::Sub<Output = T> + num::One,
{
    type Output = T;

    fn add(self, rhs: T) -> Self::Output {
        match self {
            Self::Left => rhs - T::one(),
            Self::Right => rhs + T::one(),
            Self::Stay => rhs,
        }
    }
}

macro_rules! impl_apply_direction {
    (@unsigned $T:ty) => {
        impl core::ops::Add<Direction> for $T {
            type Output = $T;

            fn add(self, rhs: Direction) -> Self::Output {
                match rhs {
                    Direction::Left => self.wrapping_sub(1),
                    Direction::Right => self.wrapping_add(1),
                    Direction::Stay => self,
                }
            }
        }

        impl core::ops::AddAssign<Direction> for $T {
            fn add_assign(&mut self, rhs: Direction) {
                *self = match rhs {
                    Direction::Left => self.wrapping_sub(1),
                    Direction::Right => self.wrapping_add(1),
                    Direction::Stay => 0,
                };
            }
        }

    };
    (@signed $T:ty) => {
        impl core::ops::Add<Direction> for $T {
            type Output = $T;

            fn add(self, rhs: Direction) -> Self::Output {
                self + rhs as $T
            }
        }

        impl core::ops::AddAssign<Direction> for $T {

            fn add_assign(&mut self, rhs: Direction) {
                *self += rhs as $T;
            }
        }

    };
    (signed: $($T:ty),* $(,)?) => {
        $(
            impl_apply_direction!(@signed $T);
        )*
    };
    (unsigned: $($T:ty),* $(,)?) => {
        $(
            impl_apply_direction!(@unsigned $T);
        )*
    };

}
impl_apply_direction!(signed: i8, i16, i32, i64, i128, isize,);
impl_apply_direction!(unsigned: u8, u16, u32, u64, u128, usize);

mod impl_from {
    use super::*;
    use crate::IntoDirection;

    macro_rules! impl_from_direction {
        ($($T:ident),*) => {
            paste::paste! {
                impl Direction {
                    $(
                        /// Converts an instance of the named type into a [Direction].
                        pub fn [<from_ $T>](value: $T) -> Self {
                            value.into_direction()
                        }
                    )*
                }
            }
            $(
                impl From<$T> for Direction {
                    fn from(value: $T) -> Self {
                        match value % 3 {
                            0 => Self::Stay,
                            1 => Self::Right,
                            _ => Self::Left,
                        }
                    }
                }
            )*
        };
        (signed: $($T:ty),*) => {
            $(
                impl From<$T> for Direction {
                    fn from(value: $T) -> Self {
                        match value % 2 {
                            -1 => Self::Left,
                            1 => Self::Right,
                            _ => Self::Stay,
                        }
                    }
                }
            )*
        };
    }

    impl_from_direction!(u8, u16, u32, u64, u128, usize);
    impl_from_direction!(signed: i8, i16, i32, i64, i128, isize);

    impl From<char> for Direction {
        fn from(value: char) -> Self {
            match value {
                'L' | 'l' => Self::Left,
                'R' | 'r' => Self::Right,
                _ => Self::Stay,
            }
        }
    }

    impl From<&str> for Direction {
        fn from(value: &str) -> Self {
            match value {
                "left" | "Left" | "LEFT" | "l" | "L" => Self::Left,
                "right" | "Right" | "RIGHT" | "r" | "R" => Self::Right,
                _ => Self::Stay,
            }
        }
    }
}
