use crate::model::item_subtype::WearableGemSubType;

pub fn from_usize(subtype: usize) -> Option<WearableGemSubType> {
    match subtype {
        1 => Some(WearableGemSubType::GemOfTeleportation),
        2 => Some(WearableGemSubType::GemOfResistCold),
        3 => Some(WearableGemSubType::GemOfResistAcid),
        4 => Some(WearableGemSubType::GemOfSeeInvisible),
        5 => Some(WearableGemSubType::GemOfStealth),
        6 => Some(WearableGemSubType::GemOfSlowDigestion),
        7 => Some(WearableGemSubType::GemOfProtectFire),
        _ => None,
    }
}

pub fn to_usize(subtype: WearableGemSubType) -> usize {
    match subtype {
        WearableGemSubType::GemOfTeleportation => 1,
        WearableGemSubType::GemOfResistCold => 2,
        WearableGemSubType::GemOfResistAcid => 3,
        WearableGemSubType::GemOfSeeInvisible => 4,
        WearableGemSubType::GemOfStealth => 5,
        WearableGemSubType::GemOfSlowDigestion => 6,
        WearableGemSubType::GemOfProtectFire => 7,
    }
}
