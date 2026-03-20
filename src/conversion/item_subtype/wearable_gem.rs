use std::convert::TryFrom;

use crate::model::item_subtype::WearableGemSubType;

/// Converts a usize to a WearableGemSubType.
/// Prefer using `WearableGemSubType::try_from(value).ok()` directly in new code.
#[deprecated]
pub fn from_usize(subtype: usize) -> Option<WearableGemSubType> {
    WearableGemSubType::try_from(subtype).ok()
}

/// Converts a WearableGemSubType to a usize.
/// Prefer using `usize::from(subtype)` or `.into()` directly in new code.
#[deprecated]
pub fn to_usize(subtype: &WearableGemSubType) -> usize {
    (*subtype).into()
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
