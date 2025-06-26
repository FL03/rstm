/*
    Appellation: mem <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Memory (mem)
//!
//!
#[doc(inline)]
pub use self::{tape::StdTape, traits::prelude::*};

pub mod cell;
pub mod snapshot;
pub mod store;
pub mod tape;

pub mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod fetch;
    pub mod memory;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::fetch::*;
        #[doc(inline)]
        pub use super::memory::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::tape::prelude::*;
    #[doc(inline)]
    pub use super::traits::prelude::*;
}
