/*
    Appellation: impl_rule_repr <module>
    Created At: 2026.01.14:23:11:59
    Contrib: @FL03
*/
use crate::rules::Rule;
use rstm_state::RawState;

impl<'a, Q, A, R, B> Rule<&'a Q, &'a A, &'a R, &'a B>
where
    Q: RawState,
    R: RawState,
{
    /// returns a new instance of the [`Rule`] with cloned elements
    pub fn cloned(&self) -> Rule<Q, A, R, B>
    where
        Q: Clone,
        A: Clone,
        R: Clone,
        B: Clone,
    {
        Rule {
            head: self.head.cloned(),
            tail: self.tail.cloned(),
        }
    }
    /// returns a new instance of the [`Rule`] with copied elements
    pub fn copied(&self) -> Rule<Q, A, R, B>
    where
        Q: Copy,
        A: Copy,
        R: Copy,
        B: Copy,
    {
        Rule {
            head: self.head.copied(),
            tail: self.tail.copied(),
        }
    }
}
