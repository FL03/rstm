/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{apply::*, convert::*, increment::*, indexing::*, symbols::*};

mod apply;
mod convert;
mod increment;
mod indexing;
mod symbols;

#[doc(hidden)]
pub mod transform;

pub(crate) mod prelude {
    pub use super::apply::*;
    pub use super::convert::*;
    pub use super::increment::*;
    pub use super::indexing::*;
    pub use super::symbols::*;
}
