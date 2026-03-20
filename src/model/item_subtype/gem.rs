use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GemSubType {
    GemOfDetectMonsters,
    GemOfDispelEvil,
    GemOfDarkness,
    GemOfAcidBalls,
    GemOfDetectInvisible,
    GemOfIdentify,
    GemOfLight,
    GemOfSummoning,
    GemOfRemoveCurse,
    GemOfAnnihilation,
    GemOfRecall,
    FineAgate,
    FineDiamond,
    RoughDiamond,
    RoughSapphire,
    FineSapphire,
    SmallBagOfOpals,
    SmallBagOfSapphires,
    SmallPouchOfDiamonds,
    LargeSackOfPearls,
    LargeSackOfSapphires,
    LargePouchOfDiamonds,
}

impl From<GemSubType> for usize {
    fn from(value: GemSubType) -> usize {
        match value {
            GemSubType::GemOfDetectMonsters => 1,
            GemSubType::GemOfDispelEvil => 2,
            GemSubType::GemOfDarkness => 3,
            GemSubType::GemOfAcidBalls => 4,
            GemSubType::GemOfDetectInvisible => 5,
            GemSubType::GemOfIdentify => 6,
            GemSubType::GemOfLight => 7,
            GemSubType::GemOfSummoning => 8,
            GemSubType::GemOfRemoveCurse => 9,
            GemSubType::GemOfAnnihilation => 10,
            GemSubType::GemOfRecall => 11,
            GemSubType::FineAgate => 257,
            GemSubType::FineDiamond => 258,
            GemSubType::RoughDiamond => 259,
            GemSubType::RoughSapphire => 260,
            GemSubType::FineSapphire => 261,
            GemSubType::SmallBagOfOpals => 262,
            GemSubType::SmallBagOfSapphires => 263,
            GemSubType::SmallPouchOfDiamonds => 264,
            GemSubType::LargeSackOfPearls => 265,
            GemSubType::LargeSackOfSapphires => 266,
            GemSubType::LargePouchOfDiamonds => 267,
        }
    }
}

impl TryFrom<usize> for GemSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(GemSubType::GemOfDetectMonsters),
            2 => Ok(GemSubType::GemOfDispelEvil),
            3 => Ok(GemSubType::GemOfDarkness),
            4 => Ok(GemSubType::GemOfAcidBalls),
            5 => Ok(GemSubType::GemOfDetectInvisible),
            6 => Ok(GemSubType::GemOfIdentify),
            7 => Ok(GemSubType::GemOfLight),
            8 => Ok(GemSubType::GemOfSummoning),
            9 => Ok(GemSubType::GemOfRemoveCurse),
            10 => Ok(GemSubType::GemOfAnnihilation),
            11 => Ok(GemSubType::GemOfRecall),
            257 => Ok(GemSubType::FineAgate),
            258 => Ok(GemSubType::FineDiamond),
            259 => Ok(GemSubType::RoughDiamond),
            260 => Ok(GemSubType::RoughSapphire),
            261 => Ok(GemSubType::FineSapphire),
            262 => Ok(GemSubType::SmallBagOfOpals),
            263 => Ok(GemSubType::SmallBagOfSapphires),
            264 => Ok(GemSubType::SmallPouchOfDiamonds),
            265 => Ok(GemSubType::LargeSackOfPearls),
            266 => Ok(GemSubType::LargeSackOfSapphires),
            267 => Ok(GemSubType::LargePouchOfDiamonds),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::GemSubType;

    #[test]
    fn test_gem_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            GemSubType::try_from(1usize).unwrap(),
            GemSubType::GemOfDetectMonsters
        );
    }

    #[test]
    fn test_gem_subtype_try_from_usize_rejects_unknown_value() {
        assert!(GemSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_gem_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(GemSubType::LargePouchOfDiamonds), 267);
    }
}
