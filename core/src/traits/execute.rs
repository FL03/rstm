/*
    Appellation: execute <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Execute {
    type Output;

    fn execute(&self) -> Result<Self::Output, crate::Error>;
}
