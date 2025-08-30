/*
    Appellation: impl_state <module>
    Contrib: @FL03
*/
use crate::state::State;

impl<Q> AsRef<Q> for State<Q> {
    fn as_ref(&self) -> &Q {
        &self.0
    }
}

impl<Q> AsMut<Q> for State<Q> {
    fn as_mut(&mut self) -> &mut Q {
        &mut self.0
    }
}

impl<Q> core::borrow::Borrow<Q> for State<Q> {
    fn borrow(&self) -> &Q {
        &self.0
    }
}

impl<Q> core::borrow::BorrowMut<Q> for State<Q> {
    fn borrow_mut(&mut self) -> &mut Q {
        &mut self.0
    }
}

impl<Q> core::ops::Deref for State<Q> {
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Q> core::ops::DerefMut for State<Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/*
 ************* Comparisons *************
*/
impl<Q> PartialEq<Q> for State<Q>
where
    Q: PartialEq,
{
    fn eq(&self, other: &Q) -> bool {
        self.0.eq(other)
    }
}

impl<'a, Q> PartialEq<&'a Q> for State<Q>
where
    Q: PartialEq,
{
    fn eq(&self, other: &&'a Q) -> bool {
        self.0.eq(*other)
    }
}

impl<'a, Q> PartialEq<&'a mut Q> for State<Q>
where
    Q: PartialEq,
{
    fn eq(&self, other: &&'a mut Q) -> bool {
        self.0.eq(*other)
    }
}

impl<Q> PartialOrd<Q> for State<Q>
where
    Q: PartialOrd,
{
    fn partial_cmp(&self, other: &Q) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<'a, Q> PartialOrd<&'a Q> for State<Q>
where
    Q: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a Q) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}

impl<'a, Q> PartialOrd<&'a mut Q> for State<Q>
where
    Q: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a mut Q) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(*other)
    }
}

/*
 ************* Conversions *************
*/
impl<Q> From<Q> for State<Q> {
    fn from(state: Q) -> Self {
        State(state)
    }
}

/*
 ************* Markers *************
*/
unsafe impl<Q> core::marker::Send for State<Q> where Q: core::marker::Send {}

unsafe impl<Q> core::marker::Sync for State<Q> where Q: core::marker::Sync {}

contained::fmt_wrapper! {
    impl State<Q> {
        Binary,
        Debug,
        Display,
        LowerExp,
        LowerHex,
        Octal,
        UpperExp,
        UpperHex
    }
}
