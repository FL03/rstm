/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{convert::*, increment::*, symbols::*};

pub(crate) mod convert;
pub(crate) mod increment;
pub(crate) mod symbols;

#[doc(hidden)]
pub mod io;
#[doc(hidden)]
pub mod transform;

pub(crate) mod prelude {
    pub use super::convert::*;
    pub use super::increment::*;
    pub use super::symbols::*;
}
