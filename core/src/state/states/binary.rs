/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use crate::State;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::EnumDiscriminants,
    strum::EnumIs,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase"),
    strum_discriminants(
        derive(serde::Deserialize, serde::Serialize),
        serde(rename_all = "lowercase")
    )
)]
#[repr(C)]
#[strum(serialize_all = "lowercase")]
#[strum_discriminants(
    name(BinState),
    derive(
        Hash,
        Ord,
        PartialOrd,
        strum::AsRefStr,
        strum::Display,
        strum::EnumCount,
        strum::EnumIs,
        strum::EnumIter,
        strum::EnumString,
        strum::VariantNames
    )
)]
pub enum BinaryState<V = String, I = V> {
    Invalid(I),
    Valid(V),
}

impl<I, V> BinaryState<V, I> {
    pub fn invalid(state: I) -> Self {
        Self::Invalid(state)
    }

    pub fn valid(state: V) -> Self {
        Self::Valid(state)
    }

    pub fn invalidate<Q>(self, state: Q) -> BinaryState<Q, Q> {
        match self {
            Self::Invalid(_) => BinaryState::Invalid(state),
            Self::Valid(_) => BinaryState::Invalid(state),
        }
    }

    pub fn kind(&self) -> BinState {
        match self {
            Self::Invalid(_) => BinState::Invalid,
            Self::Valid(_) => BinState::Valid,
        }
    }
}

impl<Q> BinaryState<Q, Q> {
    pub fn into_inner(self) -> State<Q> {
        match self {
            Self::Invalid(q) => State(q),
            Self::Valid(q) => State(q),
        }
    }

    pub fn state(&self) -> (BinState, &Q) {
        (self.kind(), self.as_ref())
    }
}



impl<Q> AsRef<Q> for BinaryState<Q, Q> {
    fn as_ref(&self) -> &Q {
        match self {
            Self::Invalid(q) => q,
            Self::Valid(q) => q,
        }
    }
}

impl<Q> AsMut<Q> for BinaryState<Q, Q> {
    fn as_mut(&mut self) -> &mut Q {
        match self {
            Self::Invalid(q) => q,
            Self::Valid(q) => q,
        }
    }
}

impl<Q> core::borrow::Borrow<Q> for BinaryState<Q, Q> {
    fn borrow(&self) -> &Q {
        self.as_ref()
    }
}

impl<Q> core::borrow::BorrowMut<Q> for BinaryState<Q, Q> {
    fn borrow_mut(&mut self) -> &mut Q {
        self.as_mut()
    }
}

impl Default for BinState {
    fn default() -> Self {
        Self::Invalid
    }
}

impl<I, V> Default for BinaryState<V, I> where I: Default {
    fn default() -> Self {
        Self::invalid(<I>::default())
    }
}

impl<Q> core::ops::Deref for BinaryState<Q, Q> {
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<Q> core::ops::DerefMut for BinaryState<Q, Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<Q> core::fmt::Display for BinaryState<Q, Q>
where
    Q: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", *self)
    }
}
