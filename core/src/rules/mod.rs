/*
    Appellation: rules <module>
    Created At: 2026.01.11:14:58:47
    Contrib: @FL03
*/
//! the `rules` module implements the [`Rule`] structure and its associated traits

#[doc(inline)]
pub use self::{head::*, rule::*, tail::*, traits::*, types::*};

mod rule;

pub mod head;
pub mod tail;

mod impls {
    mod impl_learned_rule;

    mod impl_rule;
    mod impl_rule_builder;
    mod impl_rule_ext;
    mod impl_rule_repr;

    mod impl_head;
    mod impl_head_ext;
    mod impl_head_repr;

    mod impl_tail;
    mod impl_tail_ext;
    mod impl_tail_repr;
}

mod traits {
    #[doc(inline)]
    pub use self::{convert::*, rulespace::*};

    mod convert;
    mod rulespace;
}

mod types {
    #[doc(inline)]
    pub use self::{aliases::*, direction::*};

    mod aliases;
    mod direction;
}
// prelude (local)
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::head::*;
    pub use super::rule::*;
    pub use super::tail::*;
    pub use super::traits::*;
    pub use super::types::*;
}
