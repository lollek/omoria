use crate::model::item_subtype::BagSubType;

pub fn from_usize(subtype: usize) -> Option<BagSubType> {
    match subtype {
        1 => Some(BagSubType::BagOfHolding250),
        2 => Some(BagSubType::BagOfHolding500),
        3 => Some(BagSubType::BagOfHolding1000),
        4 => Some(BagSubType::BagOfDevouring),
        5 => Some(BagSubType::BagOfHolding1500),
        _ => None,
    }
}

pub fn to_usize(subtype: BagSubType) -> usize {
    match subtype {
        BagSubType::BagOfHolding250 => 1,
        BagSubType::BagOfHolding500 => 2,
        BagSubType::BagOfHolding1000 => 3,
        BagSubType::BagOfDevouring => 4,
        BagSubType::BagOfHolding1500 => 5,
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
