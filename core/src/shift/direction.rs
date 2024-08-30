/*
    Appellation: direction <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [LinearShift]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::EnumDiscriminants,
    strum::EnumCount,
    strum::EnumIs,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), strum_discriminants(derive(serde::Deserialize, serde::Serialize)))]
#[strum(serialize_all = "lowercase")]
#[strum_discriminants(name(ShiftDirection), derive(Hash,
    Ord,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::VariantNames,), strum(serialize_all = "lowercase"))]
pub enum LinearShift<T> {
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
    Left(T),
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
    Right(T),
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
    Stay(T),
}
