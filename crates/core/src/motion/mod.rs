/*
    Appellation: mod <module>
    Created At: 2025.12.19:15:10:56
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::head_step::HeadStep;

pub mod head_step;

pub(crate) mod prelude {
    pub use super::head_step::*;
}

#[cfg(test)]
mod tests {
    use crate::{Head, Tail};

    #[test]
    fn test_head_step() {
        let mut head = Head::new(0u8, ' ');
        let tail = Tail::right(1u8, 'A');
        let step = head.step(tail);
        // define some tape
        let mut tape = [' '; 10];
        // ensure an empty tape
        assert_eq! { tape[0], ' ' }
        // define the current position
        let mut position = 0usize;

        let prev = step.move_along(&mut tape, &mut position);
        assert_eq! { position, 1 }
        assert_eq! { prev, (0u8, ' ') }
        assert_eq! { tape[0], 'A' }
    }
}
