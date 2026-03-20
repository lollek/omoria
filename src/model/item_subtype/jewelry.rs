use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JewelrySubType {
    SmallGoldPendant,
    SmallMithrilPendant,
    LargeMithrilGarterBelt,
    SmallSilverPendant,
}

impl From<JewelrySubType> for usize {
    fn from(value: JewelrySubType) -> usize {
        match value {
            JewelrySubType::SmallGoldPendant => 274,
            JewelrySubType::SmallMithrilPendant => 275,
            JewelrySubType::LargeMithrilGarterBelt => 276,
            JewelrySubType::SmallSilverPendant => 266,
        }
    }
}

impl TryFrom<usize> for JewelrySubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            274 => Ok(JewelrySubType::SmallGoldPendant),
            275 => Ok(JewelrySubType::SmallMithrilPendant),
            276 => Ok(JewelrySubType::LargeMithrilGarterBelt),
            266 => Ok(JewelrySubType::SmallSilverPendant),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::JewelrySubType;

    #[test]
    fn test_jewelry_subtype_try_from_usize_accepts_known_values() {
        assert_eq!(
            JewelrySubType::try_from(274usize).unwrap(),
            JewelrySubType::SmallGoldPendant
        );
        assert_eq!(
            JewelrySubType::try_from(275usize).unwrap(),
            JewelrySubType::SmallMithrilPendant
        );
        assert_eq!(
            JewelrySubType::try_from(276usize).unwrap(),
            JewelrySubType::LargeMithrilGarterBelt
        );
        assert_eq!(
            JewelrySubType::try_from(266usize).unwrap(),
            JewelrySubType::SmallSilverPendant
        );
    }

    #[test]
    fn test_jewelry_subtype_try_from_usize_rejects_unknown_values() {
        assert!(JewelrySubType::try_from(265usize).is_err());
        assert!(JewelrySubType::try_from(267usize).is_err());
        assert!(JewelrySubType::try_from(277usize).is_err());
    }

    #[test]
    fn test_jewelry_subtype_into_usize_returns_expected_codes() {
        assert_eq!(usize::from(JewelrySubType::SmallGoldPendant), 274);
        assert_eq!(usize::from(JewelrySubType::SmallMithrilPendant), 275);
        assert_eq!(usize::from(JewelrySubType::LargeMithrilGarterBelt), 276);
        assert_eq!(usize::from(JewelrySubType::SmallSilverPendant), 266);
    }
}
