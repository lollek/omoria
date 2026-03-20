use std::convert::TryFrom;

use crate::model::item_subtype::BeltSubType;

#[deprecated(
    note = "Use `BeltSubType::try_from(usize)` from `crate::model::item_subtype` instead."
)]
pub fn from_usize(subtype: usize) -> Option<BeltSubType> {
    BeltSubType::try_from(subtype).ok()
}

#[deprecated(note = "Use `usize::from(BeltSubType)` from `crate::model::item_subtype` instead.")]
pub fn to_usize(subtype: &BeltSubType) -> usize {
    usize::from(*subtype)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_conversion() {
        (0..1000).for_each(|i| {
            if let Some(subtype) = from_usize(i) {
                assert_eq!(i, to_usize(&subtype));
            }
        })
    }
}
