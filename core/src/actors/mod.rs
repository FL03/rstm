/*
    Appellation: mod <module>
    Created At: 2025.12.19:15:10:56
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::moving_head::MovingHead;

pub mod moving_head;

pub(crate) mod prelude {
    pub use super::moving_head::*;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Head, Tail};

    #[test]
    fn test_moving_head() {
        let mut head = Head::new(0u8, ' ');
        let tail = Tail::right(1u8, 'A');
        let moving_head = MovingHead::new(&mut head, tail);
        let mut tape = vec![' '; 10];
        // ensure an empty tape
        assert_eq! { tape[0], ' ' }
        // define the current position
        let mut position = 0usize;

        let prev = moving_head.step_on(&mut tape, &mut position);
        assert_eq! { position, 1 }
        assert_eq! { prev, (0u8, ' ') }
        assert_eq! { tape[0], 'A' }
    }
}
