/*
    Appellation: direction <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// The [AsDirection] trait provides a convience method for converting a type into a [Direction].
pub trait AsDirection {
    fn as_direction(&self) -> Direction;
}
/// The [IntoDirection] trait provides a convience method for converting a type into a [Direction].
pub trait IntoDirection {
    fn into_direction(self) -> Direction;
}

/// [Direction] enumerates the various directions a head can move, namely: left, right, and stay.
/// The included methods and implementations aim to streamline the conversion between [Direction] and other types.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIter,
    strum::VariantNames,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(i8)]
#[strum(serialize_all = "lowercase")]
pub enum Direction {
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
    Right = 1,
    #[default]
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
    Stay = 0,
}

/*
 ************* Implementations *************
*/
impl Direction {
    /// A functional constructor for [Direction::Left].
    pub fn left() -> Self {
        Self::Left
    }
    /// A functional constructor for [Direction::Right].
    pub fn right() -> Self {
        Self::Right
    }
    /// A functional constructor for [Direction::Stay].
    pub fn stay() -> Self {
        Self::Stay
    }
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
    /// Returns a [char] representation of the [direction](Direction).
    pub fn as_char(&self) -> char {
        (*self).into_char()
    }

    pub fn as_str(&self) -> &str {
        (*self).as_ref()
    }
    /// Consumes the instance, returning a [char] representation of the [direction](Direction).
    pub fn into_char(self) -> char {
        match self {
            Self::Left => 'L',
            Self::Right => 'R',
            Self::Stay => 'S',
        }
    }
    /// Applies the shift to the given position in the [direction](Direction) specified by the 
    /// current instance. This is done using the [`wrapping_add_signed`](usize::wrapping_add_signed) 
    /// method.
    pub fn apply(self, cur: usize) -> usize {
        cur.wrapping_add_signed(self as isize)
    }
}

impl<T> AsDirection for T
where
    T: Clone + Into<Direction>,
{
    fn as_direction(&self) -> Direction {
        self.clone().into()
    }
}

impl<T> IntoDirection for T
where
    T: Into<Direction>,
{
    fn into_direction(self) -> Direction {
        self.into()
    }
}

mod impl_from {
    use super::*;

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
