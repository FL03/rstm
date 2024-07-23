/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(transparent)]
pub struct State<Q = String>(pub Q);

impl<Q> State<Q> {
    pub fn new(state: Q) -> Self {
        Self(state)
    }

    pub fn into_inner(self) -> Q {
        self.0
    }

    pub fn as_ref(&self) -> &Q {
        &self.0
    }

    pub fn as_mut(&mut self) -> &mut Q {
        &mut self.0
    }
}

impl State<char> {
    pub fn is_halt(&self) -> bool {
        self.0 == 'H' || self.0 == 'h'
    }
}

impl State<&str> {
    pub fn is_halt(&self) -> bool {
        self.0.to_lowercase() == "halt"
    }
}

impl State<String> {
    pub fn is_halt(&self) -> bool {
        self.0.to_lowercase() == "halt"
    }
}

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

impl<Q> core::fmt::Binary for State<Q>
where
    Q: core::fmt::Binary,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl<Q> core::fmt::Display for State<Q>
where
    Q: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<Q> core::fmt::LowerExp for State<Q>
where
    Q: core::fmt::LowerExp,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:e}", self.0)
    }
}

impl<Q> core::fmt::LowerHex for State<Q>
where
    Q: core::fmt::LowerHex,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl<Q> core::fmt::Octal for State<Q>
where
    Q: core::fmt::Octal,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:o}", self.0)
    }
}

impl<Q> core::fmt::UpperExp for State<Q>
where
    Q: core::fmt::UpperExp,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:E}", self.0)
    }
}

impl<Q> core::fmt::UpperHex for State<Q>
where
    Q: core::fmt::UpperHex,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

unsafe impl<Q> core::marker::Send for State<Q> where Q: core::marker::Send {}

unsafe impl<Q> core::marker::Sync for State<Q> where Q: core::marker::Sync {}

impl<Q> core::cmp::PartialEq<Q> for State<Q>
where
    Q: core::cmp::PartialEq,
{
    fn eq(&self, other: &Q) -> bool {
        self.0 == *other
    }
}

impl<Q> core::cmp::PartialOrd<Q> for State<Q>
where
    Q: core::cmp::PartialOrd,
{
    fn partial_cmp(&self, other: &Q) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl<Q> From<Q> for State<Q> {
    fn from(state: Q) -> Self {
        Self(state)
    }
}
