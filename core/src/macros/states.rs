/*
    Appellation: states <macros>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[macro_export]
macro_rules! state {
    ($state:expr) => {
        $crate::State($state)
    };
}
