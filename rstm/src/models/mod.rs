/*
    Appellation: models <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::base::StdTM;

pub mod base;

pub(crate) mod prelude {
    pub use super::base::StdTM;
}
