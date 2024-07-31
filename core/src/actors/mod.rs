/*
    Appellation: actors <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{actor::Actor, exec::Executor};

pub(crate) mod actor;
pub(crate) mod exec;

#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::actor::Actor;
    pub use super::exec::Executor;
}
