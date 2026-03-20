use std::convert::TryFrom;

use crate::model::item_subtype::SongBookSubType;

/// Converts a usize to a SongBookSubType.
/// Prefer using `SongBookSubType::try_from(value).ok()` directly in new code.
#[deprecated]
pub fn from_usize(subtype: usize) -> Option<SongBookSubType> {
    SongBookSubType::try_from(subtype).ok()
}

/// Converts a SongBookSubType to a usize.
/// Prefer using `usize::from(subtype)` or `.into()` directly in new code.
#[deprecated]
pub fn to_usize(subtype: &SongBookSubType) -> usize {
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
