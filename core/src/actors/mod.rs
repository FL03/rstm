/*
    Appellation: actors <module>
    Created At: 2026.01.11:13:46:49
    Contrib: @FL03
*/
//! actors for modular Turing machine implementations
#[cfg(feature = "alloc")]
pub use self::turing_engine::*;
#[doc(inline)]
pub use self::{drivers::prelude::*, traits::*};

pub mod drivers {
    #[doc(inline)]
    pub use self::tmh::*;

    pub mod tmh;

    pub(crate) mod prelude {
        pub use super::tmh::*;
    }
}
pub mod turing_engine;

mod impls {
    mod impl_tmh;
    mod impl_tmh_engine;
    mod impl_tmh_ext;
}

mod traits {
    pub use self::{raw_driver::*, raw_engine::*};

    mod raw_driver;
    mod raw_engine;
}

// prelude (local)
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::drivers::prelude::*;
    pub use super::traits::*;
    #[cfg(feature = "alloc")]
    pub use super::turing_engine::*;
}
