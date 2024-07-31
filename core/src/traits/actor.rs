/*
    Appellation: fsm <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait RawSpace {
    type Elem;

    fn space(&self) -> Self::Elem;
}



pub trait ConfigSpace {
    type Space;

    
}