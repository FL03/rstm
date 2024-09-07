/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{convert::*, increment::*, indexing::*, symbols::*};

mod convert;
mod increment;
mod indexing;
mod symbols;

#[doc(hidden)]
pub mod container;
#[doc(hidden)]
pub mod io;
#[doc(hidden)]
pub mod transform;

pub(crate) mod prelude {
    pub use super::convert::*;
    pub use super::increment::*;
    pub use super::indexing::*;
    pub use super::symbols::*;
}
