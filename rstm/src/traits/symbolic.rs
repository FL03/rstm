/*
    Appellation: symbolic <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Symbolic {
    fn symbol(&self) -> char;
}

pub trait Alphabet {
    fn symbols(&self) -> Vec<char>;
}
