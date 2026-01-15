/*
    Appellation: rules <module>
    Created At: 2026.01.11:14:58:47
    Contrib: @FL03
*/
//! the `rules` module implements the [`Rule`] structure and its associated traits
#[doc(inline)]
pub use self::rule::*;

mod rule;

mod impls {
    mod impl_learned_rule;

    mod impl_rule;
    mod impl_rule_builder;
    mod impl_rule_ext;
    mod impl_rule_repr;
}
// prelude (local)
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) mod prelude {
    pub use super::rule::*;
}
