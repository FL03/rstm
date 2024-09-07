/*
    Appellation: models <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{base::StdTM, tmh::TMH};

pub mod base;
pub mod tmh;

pub(crate) mod prelude {
    pub use super::base::StdTM;
    pub use super::tmh::TMH;
}
