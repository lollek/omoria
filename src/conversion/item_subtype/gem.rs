use crate::model::item_subtype::GemSubType;

pub fn from_usize(subtype: usize) -> Option<GemSubType> {
    match subtype {
        1 => Some(GemSubType::GemOfDetectMonsters),
        2 => Some(GemSubType::GemOfDispelEvil),
        3 => Some(GemSubType::GemOfDarkness),
        4 => Some(GemSubType::GemOfAcidBalls),
        5 => Some(GemSubType::GemOfDetectInvisible),
        6 => Some(GemSubType::GemOfIdentify),
        7 => Some(GemSubType::GemOfLight),
        8 => Some(GemSubType::GemOfSummoning),
        9 => Some(GemSubType::GemOfRemoveCurse),
        10 => Some(GemSubType::GemOfAnnihilation),
        11 => Some(GemSubType::GemOfRecall),
        257 => Some(GemSubType::FineAgate),
        258 => Some(GemSubType::FineDiamond),
        259 => Some(GemSubType::RoughDiamond),
        260 => Some(GemSubType::RoughSapphire),
        261 => Some(GemSubType::FineSapphire),
        262 => Some(GemSubType::SmallBagOfOpals),
        263 => Some(GemSubType::SmallBagOfSapphires),
        264 => Some(GemSubType::SmallPouchOfDiamonds),
        265 => Some(GemSubType::LargeSackOfPearls),
        266 => Some(GemSubType::LargeSackOfSapphires),
        267 => Some(GemSubType::LargePouchOfDiamonds),
        _ => None,
    }
}

pub fn to_usize(subtype: GemSubType) -> usize {
    match subtype {
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
