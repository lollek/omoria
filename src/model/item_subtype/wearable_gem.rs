use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WearableGemSubType {
    GemOfTeleportation,
    GemOfResistCold,
    GemOfResistAcid,
    GemOfSeeInvisible,
    GemOfStealth,
    GemOfSlowDigestion,
    GemOfProtectFire,
}

impl From<WearableGemSubType> for usize {
    fn from(value: WearableGemSubType) -> usize {
        match value {
            WearableGemSubType::GemOfTeleportation => 1,
            WearableGemSubType::GemOfResistCold => 2,
            WearableGemSubType::GemOfResistAcid => 3,
            WearableGemSubType::GemOfSeeInvisible => 4,
            WearableGemSubType::GemOfStealth => 5,
            WearableGemSubType::GemOfSlowDigestion => 6,
            WearableGemSubType::GemOfProtectFire => 7,
        }
    }
}

impl TryFrom<usize> for WearableGemSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(WearableGemSubType::GemOfTeleportation),
            2 => Ok(WearableGemSubType::GemOfResistCold),
            3 => Ok(WearableGemSubType::GemOfResistAcid),
            4 => Ok(WearableGemSubType::GemOfSeeInvisible),
            5 => Ok(WearableGemSubType::GemOfStealth),
            6 => Ok(WearableGemSubType::GemOfSlowDigestion),
            7 => Ok(WearableGemSubType::GemOfProtectFire),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::WearableGemSubType;

    #[test]
    fn test_wearable_gem_subtype_try_from_usize_accepts_known_values() {
        assert_eq!(
            WearableGemSubType::try_from(1usize).unwrap(),
            WearableGemSubType::GemOfTeleportation
        );
        assert_eq!(
            WearableGemSubType::try_from(2usize).unwrap(),
            WearableGemSubType::GemOfResistCold
        );
        assert_eq!(
            WearableGemSubType::try_from(3usize).unwrap(),
            WearableGemSubType::GemOfResistAcid
        );
        assert_eq!(
            WearableGemSubType::try_from(4usize).unwrap(),
            WearableGemSubType::GemOfSeeInvisible
        );
        assert_eq!(
            WearableGemSubType::try_from(5usize).unwrap(),
            WearableGemSubType::GemOfStealth
        );
        assert_eq!(
            WearableGemSubType::try_from(6usize).unwrap(),
            WearableGemSubType::GemOfSlowDigestion
        );
        assert_eq!(
            WearableGemSubType::try_from(7usize).unwrap(),
            WearableGemSubType::GemOfProtectFire
        );
    }

    #[test]
    fn test_wearable_gem_subtype_try_from_usize_rejects_unknown_values() {
        assert!(WearableGemSubType::try_from(0usize).is_err());
        assert!(WearableGemSubType::try_from(8usize).is_err());
    }

    #[test]
    fn test_wearable_gem_subtype_into_usize_returns_expected_codes() {
        assert_eq!(usize::from(WearableGemSubType::GemOfTeleportation), 1);
        assert_eq!(usize::from(WearableGemSubType::GemOfResistCold), 2);
        assert_eq!(usize::from(WearableGemSubType::GemOfResistAcid), 3);
        assert_eq!(usize::from(WearableGemSubType::GemOfSeeInvisible), 4);
        assert_eq!(usize::from(WearableGemSubType::GemOfStealth), 5);
        assert_eq!(usize::from(WearableGemSubType::GemOfSlowDigestion), 6);
        assert_eq!(usize::from(WearableGemSubType::GemOfProtectFire), 7);
    }
}
