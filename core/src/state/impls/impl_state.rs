/*
    Appellation: impl_state <module>
    Contrib: @FL03
*/
use crate::state::State;

impl<Q> AsRef<Q> for State<Q> {
    fn as_ref(&self) -> &Q {
        self.get()
    }
}

impl<Q> AsMut<Q> for State<Q> {
    fn as_mut(&mut self) -> &mut Q {
        self.get_mut()
    }
}

impl<Q> core::borrow::Borrow<Q> for State<Q> {
    fn borrow(&self) -> &Q {
        self.get()
    }
}

impl<Q> core::borrow::BorrowMut<Q> for State<Q> {
    fn borrow_mut(&mut self) -> &mut Q {
        self.get_mut()
    }
}

impl<Q> core::ops::Deref for State<Q> {
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<Q> core::ops::DerefMut for State<Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
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
        self.get().eq(other)
    }
}

impl<'a, Q> PartialEq<&'a Q> for State<Q>
where
    Q: PartialEq,
{
    fn eq(&self, other: &&'a Q) -> bool {
        self.get().eq(*other)
    }
}

impl<'a, Q> PartialEq<&'a mut Q> for State<Q>
where
    Q: PartialEq,
{
    fn eq(&self, other: &&'a mut Q) -> bool {
        self.get().eq(*other)
    }
}

impl<Q> PartialOrd<Q> for State<Q>
where
    Q: PartialOrd,
{
    fn partial_cmp(&self, other: &Q) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

impl<'a, Q> PartialOrd<&'a Q> for State<Q>
where
    Q: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a Q) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(*other)
    }
}

impl<'a, Q> PartialOrd<&'a mut Q> for State<Q>
where
    Q: PartialOrd,
{
    fn partial_cmp(&self, other: &&'a mut Q) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(*other)
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

macro_rules! impl_fmt {
    ($wrap:ident<$T:ident>($($trait:ident),* $(,)?)) => {
        $(
            impl_fmt!(@impl $wrap<$T>($trait));
        )*
    };
    (@impl $wrap:ident<$T:ident>($trait:ident)) => {
        impl<$T> ::core::fmt::$trait for State<$T>
        where
            $T: ::core::fmt::$trait,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                ::core::fmt::$trait::fmt(self.get(), f)
            }
        }
    };
}

impl_fmt! {
    State<Q>(
        Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex
    )
}
