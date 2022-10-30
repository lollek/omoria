use crate::model::item_subtype::BoltSubType;

pub fn from_usize(subtype: usize) -> Option<BoltSubType> {
    match subtype {
        1 => Some(BoltSubType::Bolt),
        _ => None,
    }
}

pub fn to_usize(subtype: BoltSubType) -> usize {
    match subtype {
        BoltSubType::Bolt => 1,
    }
}

#[cfg(test)]
mod test {
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
