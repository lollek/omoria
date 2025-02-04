use crate::model::item_subtype::StaffSubType;

pub fn from_usize(subtype: usize) -> Option<StaffSubType> {
    match subtype {
        1 => Some(StaffSubType::StaffOfLight),
        2 => Some(StaffSubType::StaffOfDoorStairLocation),
        3 => Some(StaffSubType::StaffOfTrapLocation),
        4 => Some(StaffSubType::StaffOfTreasureLocation),
        5 => Some(StaffSubType::StaffOfObjectLocation),
        6 => Some(StaffSubType::StaffOfTeleportation),
        7 => Some(StaffSubType::StaffOfEarthquakes),
        8 => Some(StaffSubType::StaffOfSummoning),
        10 => Some(StaffSubType::StaffOfDestruction),
        11 => Some(StaffSubType::StaffOfStarlite),
        12 => Some(StaffSubType::StaffOfHasteMonsters),
        13 => Some(StaffSubType::StaffOfSlowMonsters),
        14 => Some(StaffSubType::StaffOfSleepMonsters),
        15 => Some(StaffSubType::StaffOfCureLightWounds),
        16 => Some(StaffSubType::StaffOfDetectInvisible),
        17 => Some(StaffSubType::StaffOfSpeed),
        18 => Some(StaffSubType::StaffOfSlowness),
        19 => Some(StaffSubType::StaffOfMassPolymorph),
        20 => Some(StaffSubType::StaffOfRemoveCurse),
        21 => Some(StaffSubType::StaffOfDetectEvil),
        22 => Some(StaffSubType::StaffOfCuring),
        23 => Some(StaffSubType::StaffOfDispelEvil),
        25 => Some(StaffSubType::StaffOfDarkness),
        26 => Some(StaffSubType::StaffOfIdentify),
        _ => None,
    }
}

pub fn to_usize(subtype: StaffSubType) -> usize {
    match subtype {
        StaffSubType::StaffOfLight => 1,
        StaffSubType::StaffOfDoorStairLocation => 2,
        StaffSubType::StaffOfTrapLocation => 3,
        StaffSubType::StaffOfTreasureLocation => 4,
        StaffSubType::StaffOfObjectLocation => 5,
        StaffSubType::StaffOfTeleportation => 6,
        StaffSubType::StaffOfEarthquakes => 7,
        StaffSubType::StaffOfSummoning => 8,
        StaffSubType::StaffOfDestruction => 10,
        StaffSubType::StaffOfStarlite => 11,
        StaffSubType::StaffOfHasteMonsters => 12,
        StaffSubType::StaffOfSlowMonsters => 13,
        StaffSubType::StaffOfSleepMonsters => 14,
        StaffSubType::StaffOfCureLightWounds => 15,
        StaffSubType::StaffOfDetectInvisible => 16,
        StaffSubType::StaffOfSpeed => 17,
        StaffSubType::StaffOfSlowness => 18,
        StaffSubType::StaffOfMassPolymorph => 19,
        StaffSubType::StaffOfRemoveCurse => 20,
        StaffSubType::StaffOfDetectEvil => 21,
        StaffSubType::StaffOfCuring => 22,
        StaffSubType::StaffOfDispelEvil => 23,
        StaffSubType::StaffOfDarkness => 25,
        StaffSubType::StaffOfIdentify => 26,
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
