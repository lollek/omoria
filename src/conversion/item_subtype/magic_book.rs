use crate::model::item_subtype::MagicBookSubType;

pub fn from_usize(subtype: usize) -> Option<MagicBookSubType> {
    match subtype {
        257 => Some(MagicBookSubType::BeginnersMagic),
        258 => Some(MagicBookSubType::Magic1),
        259 => Some(MagicBookSubType::Magic2),
        261 => Some(MagicBookSubType::MagesGuideToPower),
        _ => None,
    }
}

pub fn to_usize(subtype: MagicBookSubType) -> usize {
    match subtype {
        MagicBookSubType::BeginnersMagic => 257,
        MagicBookSubType::Magic1 => 258,
        MagicBookSubType::Magic2 => 259,
        MagicBookSubType::MagesGuideToPower => 261,
    }
}
