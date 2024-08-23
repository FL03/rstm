/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{execute::*, increment::*, symbolic::*, transform::*};

pub(crate) mod execute;
pub(crate) mod increment;
pub(crate) mod symbolic;
pub(crate) mod transform;

#[doc(hidden)]
pub mod io;

pub(crate) mod prelude {
    pub use super::execute::*;
    pub use super::increment::*;
    pub use super::symbolic::*;
    pub use super::transform::*;
}
