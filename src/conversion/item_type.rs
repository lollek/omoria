use std::convert::TryFrom;

use crate::model::ItemType;

/// Converts a usize to an ItemType.
/// Prefer using `ItemType::try_from(value).ok()` directly in new code.
#[deprecated]
pub fn from_usize(pos: usize) -> Option<ItemType> {
    ItemType::try_from(pos).ok()
}

/// Converts an ItemType to a usize.
/// Prefer using `usize::from(item_type)` or `.into()` directly in new code.
#[deprecated]
pub fn to_usize(pos: ItemType) -> usize {
    pos.into()
}
