use std::convert::TryFrom;

use crate::model::item_subtype::InstrumentSubType;

/// Converts a usize to an InstrumentSubType.
/// Prefer using `InstrumentSubType::try_from(value).ok()` directly in new code.
#[deprecated]
pub fn from_usize(subtype: usize) -> Option<InstrumentSubType> {
    InstrumentSubType::try_from(subtype).ok()
}

/// Converts an InstrumentSubType to a usize.
/// Prefer using `usize::from(subtype)` or `.into()` directly in new code.
#[deprecated]
pub fn to_usize(subtype: &InstrumentSubType) -> usize {
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
