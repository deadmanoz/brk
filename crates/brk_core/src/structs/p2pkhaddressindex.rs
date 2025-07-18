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
pub struct P2PKHAddressIndex(TypeIndex);
impl From<TypeIndex> for P2PKHAddressIndex {
    fn from(value: TypeIndex) -> Self {
        Self(value)
    }
}
impl From<P2PKHAddressIndex> for usize {
    fn from(value: P2PKHAddressIndex) -> Self {
        Self::from(*value)
    }
}
impl From<usize> for P2PKHAddressIndex {
    fn from(value: usize) -> Self {
        Self(TypeIndex::from(value))
    }
}
impl Add<usize> for P2PKHAddressIndex {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(*self + rhs)
    }
}
impl CheckedSub<P2PKHAddressIndex> for P2PKHAddressIndex {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.checked_sub(rhs.0).map(Self)
    }
}

impl Printable for P2PKHAddressIndex {
    fn to_string() -> &'static str {
        "p2pkhaddressindex"
    }

    fn to_possible_strings() -> &'static [&'static str] {
        &["pkhaddr", "p2pkhaddr", "p2pkhaddressindex"]
    }
}
