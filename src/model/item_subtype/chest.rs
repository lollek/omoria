use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChestSubType {
    //  DeadHumanBody,
    SmallWoodenChest,
    LargeWoodenChest,
    SmallIronChest,
    LargeIronChest,
    SmallSteelChest,
    LargeSteelChest,
}

impl From<ChestSubType> for usize {
    fn from(value: ChestSubType) -> usize {
        match value {
            ChestSubType::SmallWoodenChest => 1,
            ChestSubType::LargeWoodenChest => 4,
            ChestSubType::SmallIronChest => 7,
            ChestSubType::LargeIronChest => 10,
            ChestSubType::SmallSteelChest => 13,
            ChestSubType::LargeSteelChest => 16,
        }
    }
}

impl TryFrom<usize> for ChestSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ChestSubType::SmallWoodenChest),
            4 => Ok(ChestSubType::LargeWoodenChest),
            7 => Ok(ChestSubType::SmallIronChest),
            10 => Ok(ChestSubType::LargeIronChest),
            13 => Ok(ChestSubType::SmallSteelChest),
            16 => Ok(ChestSubType::LargeSteelChest),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::ChestSubType;

    #[test]
    fn test_chest_subtype_try_from_usize_accepts_known_values() {
        assert_eq!(
            ChestSubType::try_from(1usize).unwrap(),
            ChestSubType::SmallWoodenChest
        );
        assert_eq!(
            ChestSubType::try_from(4usize).unwrap(),
            ChestSubType::LargeWoodenChest
        );
        assert_eq!(
            ChestSubType::try_from(7usize).unwrap(),
            ChestSubType::SmallIronChest
        );
        assert_eq!(
            ChestSubType::try_from(10usize).unwrap(),
            ChestSubType::LargeIronChest
        );
        assert_eq!(
            ChestSubType::try_from(13usize).unwrap(),
            ChestSubType::SmallSteelChest
        );
        assert_eq!(
            ChestSubType::try_from(16usize).unwrap(),
            ChestSubType::LargeSteelChest
        );
    }

    #[test]
    fn test_chest_subtype_try_from_usize_rejects_unknown_values() {
        assert!(ChestSubType::try_from(0usize).is_err());
        assert!(ChestSubType::try_from(2usize).is_err());
        assert!(ChestSubType::try_from(17usize).is_err());
    }

    #[test]
    fn test_chest_subtype_into_usize_returns_expected_codes() {
        assert_eq!(usize::from(ChestSubType::SmallWoodenChest), 1);
        assert_eq!(usize::from(ChestSubType::LargeWoodenChest), 4);
        assert_eq!(usize::from(ChestSubType::SmallIronChest), 7);
        assert_eq!(usize::from(ChestSubType::LargeIronChest), 10);
        assert_eq!(usize::from(ChestSubType::SmallSteelChest), 13);
        assert_eq!(usize::from(ChestSubType::LargeSteelChest), 16);
    }
}
