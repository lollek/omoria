use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MiscUsableSubType {
    FlaskOfOil,
    IronSpike,
    Statue,
    SilverCross,
    GoldCross,
    MithrilCross,
    Cross,
    CorkedBottle,
}

impl From<MiscUsableSubType> for usize {
    fn from(value: MiscUsableSubType) -> usize {
        match value {
            MiscUsableSubType::FlaskOfOil => 257,
            MiscUsableSubType::IronSpike => 1,
            MiscUsableSubType::Statue => 14,
            MiscUsableSubType::SilverCross => 16,
            MiscUsableSubType::GoldCross => 17,
            MiscUsableSubType::MithrilCross => 18,
            MiscUsableSubType::Cross => 19,
            MiscUsableSubType::CorkedBottle => 21,
        }
    }
}

impl TryFrom<usize> for MiscUsableSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            257 => Ok(MiscUsableSubType::FlaskOfOil),
            1 => Ok(MiscUsableSubType::IronSpike),
            14 => Ok(MiscUsableSubType::Statue),
            16 => Ok(MiscUsableSubType::SilverCross),
            17 => Ok(MiscUsableSubType::GoldCross),
            18 => Ok(MiscUsableSubType::MithrilCross),
            19 => Ok(MiscUsableSubType::Cross),
            21 => Ok(MiscUsableSubType::CorkedBottle),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::MiscUsableSubType;

    #[test]
    fn test_misc_usable_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            MiscUsableSubType::try_from(257usize).unwrap(),
            MiscUsableSubType::FlaskOfOil
        );
    }

    #[test]
    fn test_misc_usable_subtype_try_from_usize_rejects_unknown_value() {
        assert!(MiscUsableSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_misc_usable_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(MiscUsableSubType::CorkedBottle), 21);
    }
}
