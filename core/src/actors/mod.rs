/*
    Appellation: actors <module>
    Created At: 2026.01.11:13:46:49
    Contrib: @FL03
*/
//! actors for modular Turing machine implementations
#[cfg(feature = "alloc")]
pub use self::engine_base::*;
#[doc(inline)]
pub use self::{drivers::*, traits::*};

pub mod drivers {
    #[doc(inline)]
    pub use self::tmh::*;

    mod tmh;
}

pub mod engine_base;

mod impls {
    mod impl_engine_base;
    mod impl_engine_ext;
    mod impl_engine_repr;
}

mod traits {
    pub use self::{executor::*, raw_driver::*};

    mod executor;
    mod raw_driver;
}

// prelude (local)
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::drivers::*;
    #[cfg(feature = "alloc")]
    pub use super::engine_base::*;
    pub use super::traits::*;
}
