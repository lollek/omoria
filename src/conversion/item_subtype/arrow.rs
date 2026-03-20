use std::convert::TryFrom;

use crate::model::item_subtype::ArrowSubType;

/// Converts a usize to an ArrowSubType.
/// Prefer using `ArrowSubType::try_from(value).ok()` directly in new code.
#[deprecated]
pub fn from_usize(subtype: usize) -> Option<ArrowSubType> {
    ArrowSubType::try_from(subtype).ok()
}

/// Converts an ArrowSubType to a usize.
/// Prefer using `usize::from(subtype)` or `.into()` directly in new code.
#[deprecated]
pub fn to_usize(subtype: &ArrowSubType) -> usize {
    (*subtype).into()
}
