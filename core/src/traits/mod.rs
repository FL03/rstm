/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::symbolic::*;

#[doc(hidden)]
pub mod cspace;
#[doc(hidden)]
pub mod io;
pub mod symbolic;

pub(crate) mod prelude {
    pub use super::symbolic::*;
    // pub use super::transform::*;
}
