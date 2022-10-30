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
