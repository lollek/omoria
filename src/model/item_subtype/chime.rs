use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChimeSubType {
    ChimeOfLight,
    ChimeOfDetectDoorsStairs,
    ChimeOfDetectTraps,
    ChimeOfTeleportation,
    ChimeOfThunderblast,
    ChimeOfSummonMonster,
    ChimeOfDisarming,
    ChimeOfAggravation,
    ChimeOfSlowMonster,
    ChimeOfSootheMonster,
    ChimeOfCureLightWound,
    ChimeOfChanging,
    ChimeOfRemoveCurse,
    ChimeOfCuring,
    ChimeOfDispelEvil,
    ChimeOfDarkness,
}

impl From<ChimeSubType> for usize {
    fn from(value: ChimeSubType) -> usize {
        match value {
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
}

impl TryFrom<usize> for ChimeSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ChimeSubType::ChimeOfLight),
            2 => Ok(ChimeSubType::ChimeOfDetectDoorsStairs),
            3 => Ok(ChimeSubType::ChimeOfDetectTraps),
            4 => Ok(ChimeSubType::ChimeOfTeleportation),
            5 => Ok(ChimeSubType::ChimeOfThunderblast),
            6 => Ok(ChimeSubType::ChimeOfSummonMonster),
            7 => Ok(ChimeSubType::ChimeOfDisarming),
            8 => Ok(ChimeSubType::ChimeOfAggravation),
            9 => Ok(ChimeSubType::ChimeOfSlowMonster),
            10 => Ok(ChimeSubType::ChimeOfSootheMonster),
            11 => Ok(ChimeSubType::ChimeOfCureLightWound),
            12 => Ok(ChimeSubType::ChimeOfChanging),
            13 => Ok(ChimeSubType::ChimeOfRemoveCurse),
            14 => Ok(ChimeSubType::ChimeOfCuring),
            15 => Ok(ChimeSubType::ChimeOfDispelEvil),
            16 => Ok(ChimeSubType::ChimeOfDarkness),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::ChimeSubType;

    #[test]
    fn test_chime_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            ChimeSubType::try_from(1usize).unwrap(),
            ChimeSubType::ChimeOfLight
        );
    }

    #[test]
    fn test_chime_subtype_try_from_usize_rejects_unknown_value() {
        assert!(ChimeSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_chime_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(ChimeSubType::ChimeOfDarkness), 16);
    }
}
