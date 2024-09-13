/*
    Appellation: traits <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{apply::*, convert::*, increment::*, symbols::*};

mod apply;
mod convert;
mod increment;
mod symbols;

pub(crate) mod prelude {
    pub use super::apply::*;
    pub use super::convert::*;
    pub use super::increment::*;
    pub use super::symbols::*;
}
