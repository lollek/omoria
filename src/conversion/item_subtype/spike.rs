use std::convert::TryFrom;

use crate::model::item_subtype::SpikeSubType;

/// Converts a usize to a SpikeSubType.
/// Prefer using `SpikeSubType::try_from(value).ok()` directly in new code.
#[deprecated]
pub fn from_usize(subtype: usize) -> Option<SpikeSubType> {
    SpikeSubType::try_from(subtype).ok()
}

/// Converts a SpikeSubType to a usize.
/// Prefer using `usize::from(subtype)` or `.into()` directly in new code.
#[deprecated]
pub fn to_usize(subtype: &SpikeSubType) -> usize {
    (*subtype).into()
}
