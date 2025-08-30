/*
    Appellation: rstm-core <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Rules for the rstm framework

#![allow(
    clippy::module_inception,
    clippy::new_ret_no_self,
    clippy::needless_doctest_main,
    clippy::should_implement_trait
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(allocator_api))]

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate rstm_core as rstm;

#[macro_use]
mod macros {
    #[macro_use]
    pub(crate) mod seal;
}

#[doc(inline)]
pub use self::{
    error::*,
    learned_rule::LearnedRule,
    rule::{Rule, RuleBuilder},
    traits::*,
    types::*,
};

#[cfg(feature = "std")]
#[doc(inline)]
pub use self::rule_map::RuleMap;
#[cfg(feature = "alloc")]
#[doc(inline)]
pub use self::ruleset::*;

pub mod error;
pub(crate) mod learned_rule;
pub(crate) mod rule;

#[cfg(feature = "std")]
pub mod rule_map;
#[cfg(feature = "alloc")]
pub mod ruleset;

mod traits {
    //! the traits defining compatible rules within the framework
    #[doc(inline)]
    pub use self::prelude::*;

    mod program;
    mod rules;
    mod rulespace;
    mod transition;

    mod prelude {
        #[doc(inline)]
        pub use super::program::*;
        #[doc(inline)]
        pub use super::rules::*;
        #[doc(inline)]
        pub use super::rulespace::*;
        #[doc(inline)]
        pub use super::transition::*;
    }
}

mod types {
    //! types essential to the construction of rules, programs, and other related objects
    #[doc(inline)]
    pub use self::prelude::*;

    mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
    }

    mod aliases {
        #[cfg(feature = "std")]
        use rstm_core::{Head, Tail};

        #[cfg(feature = "alloc")]
        pub(crate) type RuleVec<Q, S> = alloc::vec::Vec<crate::Rule<Q, S>>;

        /// A type alias for a [`HashMap`](std::collections::HashMap) with keys of type [`Head<Q, S>`] and values of type
        /// [`Tail<Q, S>`].
        #[cfg(feature = "std")]
        pub type HeadMap<Q = usize, S = usize> = std::collections::HashMap<Head<Q, S>, Tail<Q, S>>;
    }
}

#[doc(hidden)]
pub mod prelude {
    #[doc(no_inline)]
    pub use crate::learned_rule::LearnedRule;
    #[doc(no_inline)]
    pub use crate::rule::Rule;
    #[doc(no_inline)]
    pub use crate::traits::*;
    #[doc(no_inline)]
    pub use crate::types::*;

    #[doc(no_inline)]
    #[cfg(feature = "std")]
    pub use crate::rule_map::RuleMap;
    #[doc(no_inline)]
    #[cfg(feature = "alloc")]
    pub use crate::ruleset::Program;
}
