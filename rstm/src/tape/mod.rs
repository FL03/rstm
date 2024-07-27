/*
    Appellation: tape <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
pub use self::tape::StdTape;

pub(crate) mod tape;

#[doc(hidden)]
pub mod slider;

pub(crate) mod prelude {
    pub use super::tape::StdTape;
}
