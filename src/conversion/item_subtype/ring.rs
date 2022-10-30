use crate::model::item_subtype::RingSubType;

pub fn from_usize(subtype: usize) -> Option<RingSubType> {
    match subtype {
        1 => Some(RingSubType::RingOfGainStrength),
        2 => Some(RingSubType::RingOfGainDexterity),
        3 => Some(RingSubType::RingOfGainConstitution),
        4 => Some(RingSubType::RingOfGainIntelligence),
        7 => Some(RingSubType::RingOfSpeed1),
        35 => Some(RingSubType::RingOfSpeed2),
        8 => Some(RingSubType::RingOfSearching),
        9 => Some(RingSubType::RingOfTeleportation),
        10 => Some(RingSubType::RingOfSlowDigestion),
        11 => Some(RingSubType::RingOfResistFire),
        12 => Some(RingSubType::RingOfResistCold),
        13 => Some(RingSubType::RingOfFeatherFalling),
        14 => Some(RingSubType::RingOfAdornment1),
        15 => Some(RingSubType::RingOfAdornment2),
        16 => Some(RingSubType::RingOfWeakness),
        17 => Some(RingSubType::RingOfLordlyProtectionFire),
        18 => Some(RingSubType::RingOfLordlyProtectionAcid),
        19 => Some(RingSubType::RingOfLordlyProtectionCold),
        20 => Some(RingSubType::RingOfWoe),
        21 => Some(RingSubType::RingOfStupidity),
        22 => Some(RingSubType::RingOfIncreaseDamage),
        23 => Some(RingSubType::RingOfIncreaseToHit),
        24 => Some(RingSubType::RingOfProtection),
        25 => Some(RingSubType::RingOfAggravateMonsters),
        26 => Some(RingSubType::RingOfSeeInvisible),
        27 => Some(RingSubType::RingOfSustainStrength),
        28 => Some(RingSubType::RingOfSustainIntelligence),
        29 => Some(RingSubType::RingOfSustainWisdom),
        30 => Some(RingSubType::RingOfSustainConstitution),
        31 => Some(RingSubType::RingOfSustainDexterity),
        32 => Some(RingSubType::RingOfSustainCharisma),
        33 => Some(RingSubType::RingOfSlaying),
        34 => Some(RingSubType::RingOfGnomekind),
        _ => None,
    }
}

pub fn to_usize(subtype: RingSubType) -> usize {
    match subtype {
        RingSubType::RingOfGainStrength => 1,
        RingSubType::RingOfGainDexterity => 2,
        RingSubType::RingOfGainConstitution => 3,
        RingSubType::RingOfGainIntelligence => 4,
        RingSubType::RingOfSpeed1 => 7,
        RingSubType::RingOfSpeed2 => 35,
        RingSubType::RingOfSearching => 8,
        RingSubType::RingOfTeleportation => 9,
        RingSubType::RingOfSlowDigestion => 10,
        RingSubType::RingOfResistFire => 11,
        RingSubType::RingOfResistCold => 12,
        RingSubType::RingOfFeatherFalling => 13,
        RingSubType::RingOfAdornment1 => 14,
        RingSubType::RingOfAdornment2 => 15,
        RingSubType::RingOfWeakness => 16,
        RingSubType::RingOfLordlyProtectionFire => 17,
        RingSubType::RingOfLordlyProtectionAcid => 18,
        RingSubType::RingOfLordlyProtectionCold => 19,
        RingSubType::RingOfWoe => 20,
        RingSubType::RingOfStupidity => 21,
        RingSubType::RingOfIncreaseDamage => 22,
        RingSubType::RingOfIncreaseToHit => 23,
        RingSubType::RingOfProtection => 24,
        RingSubType::RingOfAggravateMonsters => 25,
        RingSubType::RingOfSeeInvisible => 26,
        RingSubType::RingOfSustainStrength => 27,
        RingSubType::RingOfSustainIntelligence => 28,
        RingSubType::RingOfSustainWisdom => 29,
        RingSubType::RingOfSustainConstitution => 30,
        RingSubType::RingOfSustainDexterity => 31,
        RingSubType::RingOfSustainCharisma => 32,
        RingSubType::RingOfSlaying => 33,
        RingSubType::RingOfGnomekind => 34,
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
