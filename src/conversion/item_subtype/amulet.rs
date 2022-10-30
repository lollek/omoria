use crate::model::item_subtype::AmuletSubType;

pub fn from_usize(subtype: usize) -> Option<AmuletSubType> {
    match subtype {
        11 => Some(AmuletSubType::AmuletOfAdornment1),
        12 => Some(AmuletSubType::AmuletOfAdornment2),
        5 => Some(AmuletSubType::AmuletOfWisdom),
        6 => Some(AmuletSubType::AmuletOfCharisma),
        7 => Some(AmuletSubType::AmuletOfSearching),
        8 => Some(AmuletSubType::AmuletOfTeleportation),
        9 => Some(AmuletSubType::AmuletOfSlowDigestion),
        10 => Some(AmuletSubType::AmuletOfResistAcid),
        13 => Some(AmuletSubType::AmuletOfTheMagi),
        14 => Some(AmuletSubType::AmuletOfDoom),
        30 => Some(AmuletSubType::SilverNecklace),
        40 => Some(AmuletSubType::GoldNecklace),
        50 => Some(AmuletSubType::MithrilNecklace),
        _ => None,
    }
}

pub fn to_usize(subtype: AmuletSubType) -> usize {
    match subtype {
        AmuletSubType::AmuletOfAdornment1 => 11,
        AmuletSubType::AmuletOfAdornment2 => 12,
        AmuletSubType::AmuletOfWisdom => 5,
        AmuletSubType::AmuletOfCharisma => 6,
        AmuletSubType::AmuletOfSearching => 7,
        AmuletSubType::AmuletOfTeleportation => 8,
        AmuletSubType::AmuletOfSlowDigestion => 9,
        AmuletSubType::AmuletOfResistAcid => 10,
        AmuletSubType::AmuletOfTheMagi => 13,
        AmuletSubType::AmuletOfDoom => 14,
        AmuletSubType::SilverNecklace => 30,
        AmuletSubType::GoldNecklace => 40,
        AmuletSubType::MithrilNecklace => 50,
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
