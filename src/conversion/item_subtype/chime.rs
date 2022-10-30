use crate::model::item_subtype::ChimeSubType;

pub fn from_usize(subtype: usize) -> Option<ChimeSubType> {
    match subtype {
        1 => Some(ChimeSubType::ChimeOfLight),
        2 => Some(ChimeSubType::ChimeOfDetectDoorsStairs),
        3 => Some(ChimeSubType::ChimeOfDetectTraps),
        4 => Some(ChimeSubType::ChimeOfTeleportation),
        5 => Some(ChimeSubType::ChimeOfThunderblast),
        6 => Some(ChimeSubType::ChimeOfSummonMonster),
        7 => Some(ChimeSubType::ChimeOfDisarming),
        8 => Some(ChimeSubType::ChimeOfAggravation),
        9 => Some(ChimeSubType::ChimeOfSlowMonster),
        10 => Some(ChimeSubType::ChimeOfSootheMonster),
        11 => Some(ChimeSubType::ChimeOfCureLightWound),
        12 => Some(ChimeSubType::ChimeOfChanging),
        13 => Some(ChimeSubType::ChimeOfRemoveCurse),
        14 => Some(ChimeSubType::ChimeOfCuring),
        15 => Some(ChimeSubType::ChimeOfDispelEvil),
        16 => Some(ChimeSubType::ChimeOfDarkness),
        _ => None,
    }
}

pub fn to_usize(subtype: ChimeSubType) -> usize {
    match subtype {
        ChimeSubType::ChimeOfLight => 1,
        ChimeSubType::ChimeOfDetectDoorsStairs => 2,
        ChimeSubType::ChimeOfDetectTraps => 3,
        ChimeSubType::ChimeOfTeleportation => 4,
        ChimeSubType::ChimeOfThunderblast => 5,
        ChimeSubType::ChimeOfSummonMonster => 6,
        ChimeSubType::ChimeOfDisarming => 7,
        ChimeSubType::ChimeOfAggravation => 8,
        ChimeSubType::ChimeOfSlowMonster => 9,
        ChimeSubType::ChimeOfSootheMonster => 10,
        ChimeSubType::ChimeOfCureLightWound => 11,
        ChimeSubType::ChimeOfChanging => 12,
        ChimeSubType::ChimeOfRemoveCurse => 13,
        ChimeSubType::ChimeOfCuring => 14,
        ChimeSubType::ChimeOfDispelEvil => 15,
        ChimeSubType::ChimeOfDarkness => 16,
    }
}
