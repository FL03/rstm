/*
    Appellation: fsm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Point {
    type Elem;
}

pub trait RawSpace {
    type Elem;

    private!();
}

pub trait Space: RawSpace {}

pub trait ConfigSpace {
    type Space;
}
