/*
    Appellation: container <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait RawContainer {
    type Elem;
}

/*
 ************* Implementations *************
*/

impl<T> RawContainer for Vec<T> {
    type Elem = T;
}

impl<T> RawContainer for Box<[T]> {
    type Elem = T;
}

impl<T> RawContainer for [T] {
    type Elem = T;
}

impl<T> RawContainer for &mut [T] {
    type Elem = T;
}

impl<const N: usize, T> RawContainer for &mut [T; N] {
    type Elem = T;
}
