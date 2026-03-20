use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StaffSubType {
    StaffOfLight,
    StaffOfDoorStairLocation,
    StaffOfTrapLocation,
    StaffOfTreasureLocation,
    StaffOfObjectLocation,
    StaffOfTeleportation,
    StaffOfEarthquakes,
    StaffOfSummoning,
    StaffOfDestruction,
    StaffOfStarlite,
    StaffOfHasteMonsters,
    StaffOfSlowMonsters,
    StaffOfSleepMonsters,
    StaffOfCureLightWounds,
    StaffOfDetectInvisible,
    StaffOfSpeed,
    StaffOfSlowness,
    StaffOfMassPolymorph,
    StaffOfRemoveCurse,
    StaffOfDetectEvil,
    StaffOfCuring,
    StaffOfDispelEvil,
    StaffOfDarkness,
    StaffOfIdentify,
}

impl From<StaffSubType> for usize {
    fn from(value: StaffSubType) -> usize {
        match value {
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
}

impl TryFrom<usize> for StaffSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(StaffSubType::StaffOfLight),
            2 => Ok(StaffSubType::StaffOfDoorStairLocation),
            3 => Ok(StaffSubType::StaffOfTrapLocation),
            4 => Ok(StaffSubType::StaffOfTreasureLocation),
            5 => Ok(StaffSubType::StaffOfObjectLocation),
            6 => Ok(StaffSubType::StaffOfTeleportation),
            7 => Ok(StaffSubType::StaffOfEarthquakes),
            8 => Ok(StaffSubType::StaffOfSummoning),
            10 => Ok(StaffSubType::StaffOfDestruction),
            11 => Ok(StaffSubType::StaffOfStarlite),
            12 => Ok(StaffSubType::StaffOfHasteMonsters),
            13 => Ok(StaffSubType::StaffOfSlowMonsters),
            14 => Ok(StaffSubType::StaffOfSleepMonsters),
            15 => Ok(StaffSubType::StaffOfCureLightWounds),
            16 => Ok(StaffSubType::StaffOfDetectInvisible),
            17 => Ok(StaffSubType::StaffOfSpeed),
            18 => Ok(StaffSubType::StaffOfSlowness),
            19 => Ok(StaffSubType::StaffOfMassPolymorph),
            20 => Ok(StaffSubType::StaffOfRemoveCurse),
            21 => Ok(StaffSubType::StaffOfDetectEvil),
            22 => Ok(StaffSubType::StaffOfCuring),
            23 => Ok(StaffSubType::StaffOfDispelEvil),
            25 => Ok(StaffSubType::StaffOfDarkness),
            26 => Ok(StaffSubType::StaffOfIdentify),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::StaffSubType;

    #[test]
    fn test_staff_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            StaffSubType::try_from(1usize).unwrap(),
            StaffSubType::StaffOfLight
        );
    }

    #[test]
    fn test_staff_subtype_try_from_usize_rejects_unknown_value() {
        assert!(StaffSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_staff_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(StaffSubType::StaffOfIdentify), 26);
    }
}
