use std::convert::TryFrom;

use crate::model::item_subtype::ChimeSubType;

#[deprecated]
pub fn from_usize(subtype: usize) -> Option<ChimeSubType> {
    ChimeSubType::try_from(subtype).ok()
}

#[deprecated]
pub fn to_usize(subtype: &ChimeSubType) -> usize {
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
