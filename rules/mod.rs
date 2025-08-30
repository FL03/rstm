/*
    Appellation: programs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{
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

mod learned_rule;
pub(crate) mod rule;

#[cfg(feature = "std")]
pub mod rule_map;
#[cfg(feature = "alloc")]
pub mod ruleset;

mod traits {
    #[doc(inline)]
    pub use self::prelude::*;

    mod program;
    mod transition;

    mod prelude {
        #[doc(inline)]
        pub use super::program::*;
        #[doc(inline)]
        pub use super::transition::*;
    }
}

mod types {
    #[doc(inline)]
    pub use self::prelude::*;

    mod aliases;
    mod head;
    mod tail;

    mod prelude {
        #[doc(inline)]
        pub use super::aliases::*;
        #[doc(inline)]
        pub use super::head::*;
        #[doc(inline)]
        pub use super::tail::*;
    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::learned_rule::LearnedRule;
    #[doc(inline)]
    pub use super::rule::Rule;
    #[doc(inline)]
    pub use super::traits::*;
    #[doc(inline)]
    pub use super::types::*;

    #[doc(inline)]
    #[cfg(feature = "std")]
    pub use super::rule_map::RuleMap;
    #[doc(inline)]
    #[cfg(feature = "alloc")]
    pub use super::ruleset::InstructionSet;
}
