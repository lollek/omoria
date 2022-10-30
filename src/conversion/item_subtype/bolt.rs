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
