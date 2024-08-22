/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{execute::*, symbolic::*, transform::*};

pub mod execute;
pub mod symbolic;
pub mod transform;

#[doc(hidden)]
pub mod cspace;
#[doc(hidden)]
pub mod io;

pub(crate) mod prelude {
    pub use super::execute::*;
    pub use super::symbolic::*;
    pub use super::transform::*;
}
