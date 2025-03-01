/*
    Appellation: apply <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// [ApplyOnce] describes objects capable of applying, or mapping, a function onto a value.
pub trait Apply<T, U> {
    type Output;

    fn apply<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(T) -> U;
}

/*
 ************* Implementations *************
*/
impl<T, U> Apply<T, U> for Option<T> {
    type Output = Option<U>;

    fn apply<F>(self, f: F) -> Self::Output
    where
        F: FnOnce(T) -> U,
    {
        self.map(f)
    }
}
