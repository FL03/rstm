/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::{state::State, states::*};

pub(crate) mod state;

pub(crate) mod states {
    #[doc(inline)]
    pub use self::binary::BinaryState;

    pub mod binary;
    pub mod halting;

    pub(crate) mod prelude {
        pub use super::binary::*;
    }
}

pub(crate) mod prelude {

    pub use super::state::State;
    pub use super::states::prelude::*;
}

/// 
pub trait Mode {
}

pub trait Haltable {
    fn halt(&self) -> bool;
}

impl Haltable for bool {
    fn halt(&self) -> bool {
        *self
    }
}

impl Haltable for char {
    fn halt(&self) -> bool {
        *self == 'H'
    }
}

impl Haltable for String {
    fn halt(&self) -> bool {
        self.as_str().halt()
    }
}

impl Haltable for &str {
    fn halt(&self) -> bool {
        let s = self.to_string().to_lowercase();
        matches!(s.as_str(), "h" | "H" | "stop" | "terminate")
    }
}

