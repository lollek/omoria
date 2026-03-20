use std::convert::TryFrom;

use crate::model::item_subtype::ChestSubType;

/// Converts a usize to a ChestSubType.
/// Prefer using `ChestSubType::try_from(value).ok()` directly in new code.
#[deprecated]
pub fn from_usize(subtype: usize) -> Option<ChestSubType> {
    ChestSubType::try_from(subtype).ok()
}

/// Converts a ChestSubType to a usize.
/// Prefer using `usize::from(subtype)` or `.into()` directly in new code.
#[deprecated]
pub fn to_usize(chest: &ChestSubType) -> usize {
    (*chest).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_double_conversion() {
        (0..1000).for_each(|i| {
            if let Some(subtype) = from_usize(i) {
                assert_eq!(i, to_usize(&subtype));
            }
        })
    }
}
