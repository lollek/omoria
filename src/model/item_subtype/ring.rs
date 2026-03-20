use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RingSubType {
    RingOfGainStrength,
    RingOfGainDexterity,
    RingOfGainConstitution,
    RingOfGainIntelligence,
    RingOfSpeed1,
    RingOfSpeed2,
    RingOfSearching,
    RingOfTeleportation,
    RingOfSlowDigestion,
    RingOfResistFire,
    RingOfResistCold,
    RingOfFeatherFalling,
    RingOfAdornment1,
    RingOfAdornment2,
    RingOfWeakness,
    RingOfLordlyProtectionFire,
    RingOfLordlyProtectionAcid,
    RingOfLordlyProtectionCold,
    RingOfWoe,
    RingOfStupidity,
    RingOfIncreaseDamage,
    RingOfIncreaseToHit,
    RingOfProtection,
    RingOfAggravateMonsters,
    RingOfSeeInvisible,
    RingOfSustainStrength,
    RingOfSustainIntelligence,
    RingOfSustainWisdom,
    RingOfSustainConstitution,
    RingOfSustainDexterity,
    RingOfSustainCharisma,
    RingOfSlaying,
    RingOfGnomekind,
}

impl From<RingSubType> for usize {
    fn from(value: RingSubType) -> usize {
        match value {
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
}

impl TryFrom<usize> for RingSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(RingSubType::RingOfGainStrength),
            2 => Ok(RingSubType::RingOfGainDexterity),
            3 => Ok(RingSubType::RingOfGainConstitution),
            4 => Ok(RingSubType::RingOfGainIntelligence),
            7 => Ok(RingSubType::RingOfSpeed1),
            35 => Ok(RingSubType::RingOfSpeed2),
            8 => Ok(RingSubType::RingOfSearching),
            9 => Ok(RingSubType::RingOfTeleportation),
            10 => Ok(RingSubType::RingOfSlowDigestion),
            11 => Ok(RingSubType::RingOfResistFire),
            12 => Ok(RingSubType::RingOfResistCold),
            13 => Ok(RingSubType::RingOfFeatherFalling),
            14 => Ok(RingSubType::RingOfAdornment1),
            15 => Ok(RingSubType::RingOfAdornment2),
            16 => Ok(RingSubType::RingOfWeakness),
            17 => Ok(RingSubType::RingOfLordlyProtectionFire),
            18 => Ok(RingSubType::RingOfLordlyProtectionAcid),
            19 => Ok(RingSubType::RingOfLordlyProtectionCold),
            20 => Ok(RingSubType::RingOfWoe),
            21 => Ok(RingSubType::RingOfStupidity),
            22 => Ok(RingSubType::RingOfIncreaseDamage),
            23 => Ok(RingSubType::RingOfIncreaseToHit),
            24 => Ok(RingSubType::RingOfProtection),
            25 => Ok(RingSubType::RingOfAggravateMonsters),
            26 => Ok(RingSubType::RingOfSeeInvisible),
            27 => Ok(RingSubType::RingOfSustainStrength),
            28 => Ok(RingSubType::RingOfSustainIntelligence),
            29 => Ok(RingSubType::RingOfSustainWisdom),
            30 => Ok(RingSubType::RingOfSustainConstitution),
            31 => Ok(RingSubType::RingOfSustainDexterity),
            32 => Ok(RingSubType::RingOfSustainCharisma),
            33 => Ok(RingSubType::RingOfSlaying),
            34 => Ok(RingSubType::RingOfGnomekind),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::RingSubType;

    #[test]
    fn test_ring_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            RingSubType::try_from(1usize).unwrap(),
            RingSubType::RingOfGainStrength
        );
    }

    #[test]
    fn test_ring_subtype_try_from_usize_rejects_unknown_value() {
        assert!(RingSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_ring_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(RingSubType::RingOfSpeed2), 35);
    }
}
