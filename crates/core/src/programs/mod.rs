/*
    Appellation: program <module>
    Created At: 2026.01.11:12:29:14
    Contrib: @FL03
*/
//! the `program` module implements the core `Program` structure and its associated traits
#[cfg(feature = "alloc")]
#[allow(deprecated)]
pub use self::program::ProgramO;
#[doc(inline)]
pub use self::{instruction_set::*, traits::*};

mod instruction_set;
mod program;

mod impls {
    mod impl_program;
    mod impl_program_ext;

    mod impl_instruction_set;
    mod impl_instruction_set_ext;
    mod impl_instruction_set_repr;
}

mod traits {
    #[doc(inline)]
    pub use self::ruleset::*;

    mod ruleset;
}
// mod types {
//     #[doc(inline)]
//     pub use self::aliases::*;

//     mod aliases;
// }

#[doc(hidden)]
pub(crate) mod prelude {
    pub use super::instruction_set::*;
    pub use super::traits::*;
}
