/*
    Appellation: impl_rule_repr <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::rule::Rule;

impl<'a, 'b, Q, A> Rule<&'a Q, &'b A> {
    pub fn cloned(&self) -> Rule<Q, A>
    where
        Q: Clone,
        A: Clone,
    {
        Rule {
            head: self.head.cloned(),
            tail: self.tail.cloned(),
        }
    }

    pub fn copied(&self) -> Rule<Q, A>
    where
        Q: Copy,
        A: Copy,
    {
        Rule {
            head: self.head.copied(),
            tail: self.tail.copied(),
        }
    }
}
