use std::convert::TryFrom;

use crate::model::item_subtype::HardArmorSubType;

#[deprecated]
pub fn from_usize(subtype: usize) -> Option<HardArmorSubType> {
    HardArmorSubType::try_from(subtype).ok()
}

#[deprecated]
pub fn to_usize(subtype: &HardArmorSubType) -> usize {
    usize::from(*subtype)
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
