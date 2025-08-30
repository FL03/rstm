/*
    Appellation: rstm-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! The core modules for the `rstm` framework, providing a suite of fundamental abstractions
//! and primitives for creating and managing state machines and related constructs.
#![allow(
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_doctest_main,
    clippy::self_named_constructors,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(inline)]
pub use rstm_state as state;

#[doc(inline)]
pub use self::{
    error::{Error, Result},
    ops::*,
    state::{RawState, State},
    traits::*,
    types::*,
};

#[macro_use]
pub(crate) mod macros {
    #[macro_use]
    pub mod seal;
}

pub mod error;

pub mod ops {
    //! this modules defines additional operations used throughout the crate
    #[doc(inline)]
    pub use self::prelude::*;

    mod apply;
    mod increment;

    mod prelude {
        #[doc(inline)]
        pub use super::apply::*;
        #[doc(inline)]
        pub use super::increment::*;
    }
}

pub mod traits {
    /// this modules provides various traits used throughout the library
    pub use self::prelude::*;

    mod convert;
    mod symbols;

    mod prelude {
        #[doc(inline)]
        pub use super::convert::*;
        #[doc(inline)]
        pub use super::symbols::*;
    }
}

pub mod types {
    //! the `types` module provides various types used throughout the library, including
    //! [`Direction`], [`Head`], and [`Tail`].
    #[doc(inline)]
    pub use self::prelude::*;

    mod direction;
    mod head;
    mod tail;

    mod prelude {
        #[doc(inline)]
        pub use super::direction::*;
        #[doc(inline)]
        pub use super::head::*;
        #[doc(inline)]
        pub use super::tail::*;
    }
}

#[doc(hidden)]
pub mod prelude {
    #[doc(no_inline)]
    pub use rstm_state::prelude::*;

    pub use crate::ops::*;
    pub use crate::traits::*;
    pub use crate::types::*;
}
