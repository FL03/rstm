/*
    appellation: state <module>
    authors: @FL03
*/

#[macro_export]
macro_rules! state {
    ($state:expr) => {
        $crate::State($state)
    };
}
