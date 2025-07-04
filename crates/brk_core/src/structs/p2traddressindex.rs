use std::ops::Add;

use derive_deref::{Deref, DerefMut};
use serde::Serialize;
use zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout};

use crate::{CheckedSub, Printable, TypeIndex};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Deref,
    DerefMut,
    Default,
    FromBytes,
    Immutable,
    IntoBytes,
    KnownLayout,
    Serialize,
)]
pub struct P2TRAddressIndex(TypeIndex);
impl From<TypeIndex> for P2TRAddressIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2TRAddressIndex> for usize {
    fn from(value: P2TRAddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2TRAddressIndex {
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl Add<usize> for P2TRAddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2TRAddressIndex> for P2TRAddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Printable for P2TRAddressIndex {
    fn to_string() -> &'static str {
        "p2traddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["traddr", "p2traddr", "p2traddressindex"]
    }
}
