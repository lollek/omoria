use std::convert::TryFrom;

use crate::model::item_subtype::LightSourceSubType;

/// Converts a usize to a LightSourceSubType.
/// Prefer using `LightSourceSubType::try_from(value).ok()` directly in new code.
#[deprecated]
pub fn from_usize(subtype: usize) -> Option<LightSourceSubType> {
    LightSourceSubType::try_from(subtype).ok()
}

/// Converts a LightSourceSubType to a usize.
/// Prefer using `usize::from(subtype)` or `.into()` directly in new code.
#[deprecated]
pub fn to_usize(light_source: &LightSourceSubType) -> usize {
    (*light_source).into()
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
