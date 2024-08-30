/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{convert::*, increment::*, symbolic::*, transform::*};

pub(crate) mod convert;
pub(crate) mod increment;
pub(crate) mod symbolic;
pub(crate) mod transform;

#[doc(hidden)]
pub mod io;

pub(crate) mod prelude {
    pub use super::convert::*;
    pub use super::increment::*;
    pub use super::symbolic::*;
    pub use super::transform::*;
}
