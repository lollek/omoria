use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BagSubType {
    BagOfHolding250,
    BagOfHolding500,
    BagOfHolding1000,
    BagOfHolding1500,
    BagOfDevouring,
}

impl From<BagSubType> for usize {
    fn from(value: BagSubType) -> usize {
        match value {
            BagSubType::BagOfHolding250 => 1,
            BagSubType::BagOfHolding500 => 2,
            BagSubType::BagOfHolding1000 => 3,
            BagSubType::BagOfDevouring => 4,
            BagSubType::BagOfHolding1500 => 5,
        }
    }
}

impl TryFrom<usize> for BagSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BagSubType::BagOfHolding250),
            2 => Ok(BagSubType::BagOfHolding500),
            3 => Ok(BagSubType::BagOfHolding1000),
            4 => Ok(BagSubType::BagOfDevouring),
            5 => Ok(BagSubType::BagOfHolding1500),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::BagSubType;

    #[test]
    fn test_bag_subtype_try_from_usize_accepts_known_values() {
        assert_eq!(
            BagSubType::try_from(1usize).unwrap(),
            BagSubType::BagOfHolding250
        );
        assert_eq!(
            BagSubType::try_from(2usize).unwrap(),
            BagSubType::BagOfHolding500
        );
        assert_eq!(
            BagSubType::try_from(3usize).unwrap(),
            BagSubType::BagOfHolding1000
        );
        assert_eq!(
            BagSubType::try_from(4usize).unwrap(),
            BagSubType::BagOfDevouring
        );
        assert_eq!(
            BagSubType::try_from(5usize).unwrap(),
            BagSubType::BagOfHolding1500
        );
    }

    #[test]
    fn test_bag_subtype_try_from_usize_rejects_unknown_values() {
        assert!(BagSubType::try_from(0usize).is_err());
        assert!(BagSubType::try_from(6usize).is_err());
    }

    #[test]
    fn test_bag_subtype_into_usize_returns_expected_codes() {
        assert_eq!(usize::from(BagSubType::BagOfHolding250), 1);
        assert_eq!(usize::from(BagSubType::BagOfHolding500), 2);
        assert_eq!(usize::from(BagSubType::BagOfHolding1000), 3);
        assert_eq!(usize::from(BagSubType::BagOfDevouring), 4);
        assert_eq!(usize::from(BagSubType::BagOfHolding1500), 5);
    }
}
