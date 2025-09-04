/*
    Appellation: actor <test>
    Created At: 2025.08.30:00:33:41
    Contrib: @FL03
*/

fn add<A, B, C>(a: A, b: B) -> C
where
    A: core::ops::Add<B, Output = C>,
{
    a + b
}


