/*
    Appellation: transition <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub struct Entry<Q, S> {
    key: Head<Q, S>,
    value: Tail<Q, S>,
}