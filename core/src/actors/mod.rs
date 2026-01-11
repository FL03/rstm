/*
    Appellation: actors <module>
    Created At: 2026.01.11:13:46:49
    Contrib: @FL03
*/
//! actors for modular Turing machine implementations
#[doc(inline)]
pub use self::{engine::*, tmh::TMH, traits::*};

pub mod engine;
pub mod tmh;

mod impls {
    mod impl_deprecated;
    mod impl_tmh;
    mod impl_tmh_ext;
    mod impl_turing_engine;
}

mod traits {
    //! traits for actors
    #[doc(inline)]
    pub use self::actor::*;
    mod actor;
}
// prelude (local)
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::engine::*;
    pub use super::tmh::TMH;
    pub use super::traits::*;
}
