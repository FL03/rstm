/*
    Appellation: impl_ext <state>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::state::State;

/*
 ************* References *************
*/
impl<Q> core::convert::AsRef<Q> for State<Q> {
    fn as_ref(&self) -> &Q {
        &self.0
    }
}

impl<Q> core::convert::AsMut<Q> for State<Q> {
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
impl<Q> core::cmp::PartialEq<Q> for State<Q>
where
    Q: core::cmp::PartialEq,
{
    fn eq(&self, other: &Q) -> bool {
        self.get_ref().eq(other)
    }
}

impl<Q> core::cmp::PartialOrd<Q> for State<Q>
where
    Q: core::cmp::PartialOrd<Q>,
{
    fn partial_cmp(&self, other: &Q) -> Option<core::cmp::Ordering> {
        self.get_ref().partial_cmp(other)
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

/*
 ************* Formatting *************
*/
macro_rules! impl_fmt {
    ($($trait:ident),*) => {
        $(
            impl<Q> core::fmt::$trait for State<Q>
            where
                Q: core::fmt::$trait,
            {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    core::fmt::$trait::fmt(&self.0, f)
                }
            }
        )*
    };
}

impl_fmt!(Binary, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex);
