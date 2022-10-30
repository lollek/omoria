use crate::model::item_subtype::JewelrySubType;

pub fn from_usize(subtype: usize) -> Option<JewelrySubType> {
    match subtype {
        274 => Some(JewelrySubType::SmallGoldPendant),
        275 => Some(JewelrySubType::SmallMithrilPendant),
        276 => Some(JewelrySubType::LargeMithrilGarterBelt),
        266 => Some(JewelrySubType::SmallSilverPendant),
        _ => None,
    }
}

pub fn to_usize(subtype: JewelrySubType) -> usize {
    match subtype {
        JewelrySubType::SmallGoldPendant => 274,
        JewelrySubType::SmallMithrilPendant => 275,
        JewelrySubType::LargeMithrilGarterBelt => 276,
        JewelrySubType::SmallSilverPendant => 266,
    }
}
