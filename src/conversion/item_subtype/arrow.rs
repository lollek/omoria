use crate::model::item_subtype::ArrowSubType;

pub fn from_usize(subtype: usize) -> Option<ArrowSubType> {
    match subtype {
        1 => Some(ArrowSubType::Arrow),
        _ => None,
    }
}

pub fn to_usize(subtype: ArrowSubType) -> usize {
    match subtype {
        ArrowSubType::Arrow => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_conversion() {
        (0..1000).for_each(|i| {
            if let Some(subtype) = from_usize(i) {
                assert_eq!(i, to_usize(subtype));
            }
        })
    }
}
