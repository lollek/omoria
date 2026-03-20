use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LightSourceSubType {
    WoodenTorch,
    BrassLantern,
    MagicTorch,
    MagicLantern,
}

impl From<LightSourceSubType> for usize {
    fn from(value: LightSourceSubType) -> usize {
        match value {
            LightSourceSubType::BrassLantern => 1,
            LightSourceSubType::WoodenTorch => 13,
            LightSourceSubType::MagicTorch => 30,
            LightSourceSubType::MagicLantern => 17,
        }
    }
}

impl TryFrom<usize> for LightSourceSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(LightSourceSubType::BrassLantern),
            13 => Ok(LightSourceSubType::WoodenTorch),
            30 => Ok(LightSourceSubType::MagicTorch),
            17 => Ok(LightSourceSubType::MagicLantern),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::LightSourceSubType;

    #[test]
    fn test_light_source_subtype_try_from_usize_accepts_known_values() {
        assert_eq!(
            LightSourceSubType::try_from(1usize).unwrap(),
            LightSourceSubType::BrassLantern
        );
        assert_eq!(
            LightSourceSubType::try_from(13usize).unwrap(),
            LightSourceSubType::WoodenTorch
        );
        assert_eq!(
            LightSourceSubType::try_from(30usize).unwrap(),
            LightSourceSubType::MagicTorch
        );
        assert_eq!(
            LightSourceSubType::try_from(17usize).unwrap(),
            LightSourceSubType::MagicLantern
        );
    }

    #[test]
    fn test_light_source_subtype_try_from_usize_rejects_unknown_values() {
        assert!(LightSourceSubType::try_from(0usize).is_err());
        assert!(LightSourceSubType::try_from(2usize).is_err());
    }

    #[test]
    fn test_light_source_subtype_into_usize_returns_expected_codes() {
        assert_eq!(usize::from(LightSourceSubType::BrassLantern), 1);
        assert_eq!(usize::from(LightSourceSubType::WoodenTorch), 13);
        assert_eq!(usize::from(LightSourceSubType::MagicTorch), 30);
        assert_eq!(usize::from(LightSourceSubType::MagicLantern), 17);
    }
}
