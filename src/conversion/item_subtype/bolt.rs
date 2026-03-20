use std::convert::TryFrom;

use crate::model::item_subtype::BoltSubType;

/// Converts a usize to a BoltSubType.
/// Prefer using `BoltSubType::try_from(value).ok()` directly in new code.
#[deprecated]
pub fn from_usize(subtype: usize) -> Option<BoltSubType> {
    BoltSubType::try_from(subtype).ok()
}

/// Converts a BoltSubType to a usize.
/// Prefer using `usize::from(subtype)` or `.into()` directly in new code.
#[deprecated]
pub fn to_usize(subtype: &BoltSubType) -> usize {
    (*subtype).into()
}
