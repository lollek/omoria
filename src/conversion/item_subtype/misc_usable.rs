use crate::model::item_subtype::MiscUsableSubType;

pub fn from_usize(subtype: usize) -> Option<MiscUsableSubType> {
    match subtype {
        257 => Some(MiscUsableSubType::FlaskOfOil),
        1 => Some(MiscUsableSubType::IronSpike),
        14 => Some(MiscUsableSubType::Statue),
        16 => Some(MiscUsableSubType::SilverCross),
        17 => Some(MiscUsableSubType::GoldCross),
        18 => Some(MiscUsableSubType::MithrilCross),
        19 => Some(MiscUsableSubType::Cross),
        21 => Some(MiscUsableSubType::CorkedBottle),
        _ => None,
    }
}

pub fn to_usize(misc_usable: MiscUsableSubType) -> usize {
    match misc_usable {
        MiscUsableSubType::FlaskOfOil => 257,
        MiscUsableSubType::IronSpike => 1,
        MiscUsableSubType::Statue => 14,
        MiscUsableSubType::SilverCross => 16,
        MiscUsableSubType::GoldCross => 17,
        MiscUsableSubType::MithrilCross => 18,
        MiscUsableSubType::Cross => 19,
        MiscUsableSubType::CorkedBottle => 21,
    }
}
