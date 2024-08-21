/*
    Appellation: io <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait RawBuf {
    type Elem;

    /// Returns an immutable reference to the buffer as a slice
    fn as_slice(&self) -> &[Self::Elem];
    /// Returns a mutable reference to the buffer as a slice
    fn as_mut_slice(&mut self) -> &mut [Self::Elem];
}

/// The `Read` trait provides a way to read bytes from a source
pub trait Read<B>
where
    B: RawBuf,
{
    type Output;

    fn read(&mut self, buf: &mut B) -> Self::Output;
}

pub trait Write<B>
where
    B: RawBuf,
{
    type Output;

    fn write(&mut self, buf: &mut B) -> Self::Output;
}

/*
 ************* Implementations *************
*/
impl<T> RawBuf for [T] {
    type Elem = T;

    fn as_slice(&self) -> &[Self::Elem] {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        self
    }
}

impl<T> RawBuf for Vec<T> {
    type Elem = T;

    fn as_slice(&self) -> &[Self::Elem] {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Elem] {
        self
    }
}
