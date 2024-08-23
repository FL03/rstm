/*
    Appellation: cspace <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Configuration Space (`C`)
//! 
//! Configuration space `C` is the set of all possible configurations `q` of a system where 
//! `q in Q`.

/// The [`Point`] trait describes a point in a space.
pub trait Point {
    type Elem;
}

/// [RawSpace] describes 
pub unsafe trait RawSpace<T> {
    type Elem: Point<Elem = T>;

}

/// [Space] describes a space.
pub trait Space<T>: RawSpace<T> {

}

