/*
    Appellation: models <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::turing::Turm;

pub mod turing;

pub(crate) mod prelude {
    pub use super::turing::Turm;
}