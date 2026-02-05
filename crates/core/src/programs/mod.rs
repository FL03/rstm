/*
    Appellation: program <module>
    Created At: 2026.01.11:12:29:14
    Contrib: @FL03
*/
//! This module provides the [`ProgramBase`] implementation along with its associated aliases,
//! supporting traits, and more.
#[cfg(feature = "alloc")]
#[allow(deprecated)]
pub use self::instruction_set::InstructionSet;
#[doc(inline)]
pub use self::{program_base::ProgramBase, traits::*, types::*};

mod instruction_set;
mod program_base;

mod impls {
    mod imp_program_base;
    mod imp_program_base_ext;
    mod imp_program_base_repr;
}

mod traits {
    #[doc(inline)]
    pub use self::ruleset::*;

    mod ruleset;
}

mod types {
    #[doc(inline)]
    pub use self::aliases::*;

    mod aliases;
}

#[doc(hidden)]
pub(crate) mod prelude {
    pub use super::program_base::*;
    pub use super::traits::*;
    pub use super::types::*;
}
