use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmuletSubType {
    AmuletOfAdornment1,
    AmuletOfAdornment2,
    AmuletOfWisdom,
    AmuletOfCharisma,
    AmuletOfSearching,
    AmuletOfTeleportation,
    AmuletOfSlowDigestion,
    AmuletOfResistAcid,
    AmuletOfTheMagi,
    AmuletOfDoom,
    SilverNecklace,
    GoldNecklace,
    MithrilNecklace,
}

impl From<AmuletSubType> for usize {
    fn from(value: AmuletSubType) -> usize {
        match value {
            AmuletSubType::AmuletOfWisdom => 5,
            AmuletSubType::AmuletOfCharisma => 6,
            AmuletSubType::AmuletOfSearching => 7,
            AmuletSubType::AmuletOfTeleportation => 8,
            AmuletSubType::AmuletOfSlowDigestion => 9,
            AmuletSubType::AmuletOfResistAcid => 10,
            AmuletSubType::AmuletOfAdornment1 => 11,
            AmuletSubType::AmuletOfAdornment2 => 12,
            AmuletSubType::AmuletOfTheMagi => 13,
            AmuletSubType::AmuletOfDoom => 14,
            AmuletSubType::SilverNecklace => 30,
            AmuletSubType::GoldNecklace => 40,
            AmuletSubType::MithrilNecklace => 50,
        }
    }
}

impl TryFrom<usize> for AmuletSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            5 => Ok(AmuletSubType::AmuletOfWisdom),
            6 => Ok(AmuletSubType::AmuletOfCharisma),
            7 => Ok(AmuletSubType::AmuletOfSearching),
            8 => Ok(AmuletSubType::AmuletOfTeleportation),
            9 => Ok(AmuletSubType::AmuletOfSlowDigestion),
            10 => Ok(AmuletSubType::AmuletOfResistAcid),
            11 => Ok(AmuletSubType::AmuletOfAdornment1),
            12 => Ok(AmuletSubType::AmuletOfAdornment2),
            13 => Ok(AmuletSubType::AmuletOfTheMagi),
            14 => Ok(AmuletSubType::AmuletOfDoom),
            30 => Ok(AmuletSubType::SilverNecklace),
            40 => Ok(AmuletSubType::GoldNecklace),
            50 => Ok(AmuletSubType::MithrilNecklace),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::AmuletSubType;

    #[test]
    fn test_amulet_subtype_try_from_usize_accepts_known_values() {
        assert_eq!(
            AmuletSubType::try_from(5usize).unwrap(),
            AmuletSubType::AmuletOfWisdom
        );
        assert_eq!(
            AmuletSubType::try_from(6usize).unwrap(),
            AmuletSubType::AmuletOfCharisma
        );
        assert_eq!(
            AmuletSubType::try_from(7usize).unwrap(),
            AmuletSubType::AmuletOfSearching
        );
        assert_eq!(
            AmuletSubType::try_from(8usize).unwrap(),
            AmuletSubType::AmuletOfTeleportation
        );
        assert_eq!(
            AmuletSubType::try_from(9usize).unwrap(),
            AmuletSubType::AmuletOfSlowDigestion
        );
        assert_eq!(
            AmuletSubType::try_from(10usize).unwrap(),
            AmuletSubType::AmuletOfResistAcid
        );
        assert_eq!(
            AmuletSubType::try_from(11usize).unwrap(),
            AmuletSubType::AmuletOfAdornment1
        );
        assert_eq!(
            AmuletSubType::try_from(12usize).unwrap(),
            AmuletSubType::AmuletOfAdornment2
        );
        assert_eq!(
            AmuletSubType::try_from(13usize).unwrap(),
            AmuletSubType::AmuletOfTheMagi
        );
        assert_eq!(
            AmuletSubType::try_from(14usize).unwrap(),
            AmuletSubType::AmuletOfDoom
        );
        assert_eq!(
            AmuletSubType::try_from(30usize).unwrap(),
            AmuletSubType::SilverNecklace
        );
        assert_eq!(
            AmuletSubType::try_from(40usize).unwrap(),
            AmuletSubType::GoldNecklace
        );
        assert_eq!(
            AmuletSubType::try_from(50usize).unwrap(),
            AmuletSubType::MithrilNecklace
        );
    }

    #[test]
    fn test_amulet_subtype_try_from_usize_rejects_unknown_values() {
        assert!(AmuletSubType::try_from(4usize).is_err());
        assert!(AmuletSubType::try_from(15usize).is_err());
        assert!(AmuletSubType::try_from(31usize).is_err());
    }

    #[test]
    fn test_amulet_subtype_into_usize_returns_expected_codes() {
        assert_eq!(usize::from(AmuletSubType::AmuletOfWisdom), 5);
        assert_eq!(usize::from(AmuletSubType::AmuletOfCharisma), 6);
        assert_eq!(usize::from(AmuletSubType::AmuletOfSearching), 7);
        assert_eq!(usize::from(AmuletSubType::AmuletOfTeleportation), 8);
        assert_eq!(usize::from(AmuletSubType::AmuletOfSlowDigestion), 9);
        assert_eq!(usize::from(AmuletSubType::AmuletOfResistAcid), 10);
        assert_eq!(usize::from(AmuletSubType::AmuletOfAdornment1), 11);
        assert_eq!(usize::from(AmuletSubType::AmuletOfAdornment2), 12);
        assert_eq!(usize::from(AmuletSubType::AmuletOfTheMagi), 13);
        assert_eq!(usize::from(AmuletSubType::AmuletOfDoom), 14);
        assert_eq!(usize::from(AmuletSubType::SilverNecklace), 30);
        assert_eq!(usize::from(AmuletSubType::GoldNecklace), 40);
        assert_eq!(usize::from(AmuletSubType::MithrilNecklace), 50);
    }
}
