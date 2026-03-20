use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SlingAmmoSubType {
    RoundedPebble,
    IronShot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoltSubType {
    Bolt,
}

impl From<BoltSubType> for usize {
    fn from(value: BoltSubType) -> usize {
        match value {
            BoltSubType::Bolt => 1,
        }
    }
}

impl TryFrom<usize> for BoltSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BoltSubType::Bolt),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArrowSubType {
    Arrow,
}

impl From<ArrowSubType> for usize {
    fn from(value: ArrowSubType) -> usize {
        match value {
            ArrowSubType::Arrow => 1,
        }
    }
}

impl TryFrom<usize> for ArrowSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ArrowSubType::Arrow),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpikeSubType {
    IronSpike,
}

impl From<SpikeSubType> for usize {
    fn from(value: SpikeSubType) -> usize {
        match value {
            SpikeSubType::IronSpike => 1,
        }
    }
}

impl TryFrom<usize> for SpikeSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SpikeSubType::IronSpike),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::ArrowSubType;
    use super::BoltSubType;
    use super::SpikeSubType;

    #[test]
    fn test_arrow_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(ArrowSubType::try_from(1usize).unwrap(), ArrowSubType::Arrow);
    }

    #[test]
    fn test_arrow_subtype_try_from_usize_rejects_unknown_values() {
        assert!(ArrowSubType::try_from(0usize).is_err());
        assert!(ArrowSubType::try_from(2usize).is_err());
    }

    #[test]
    fn test_arrow_subtype_into_usize_returns_expected_code() {
        let code: usize = ArrowSubType::Arrow.into();
        assert_eq!(code, 1);
    }

    #[test]
    fn test_bolt_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(BoltSubType::try_from(1usize).unwrap(), super::BoltSubType::Bolt);
    }

    #[test]
    fn test_bolt_subtype_try_from_usize_rejects_unknown_values() {
        assert!(BoltSubType::try_from(0usize).is_err());
        assert!(BoltSubType::try_from(2usize).is_err());
    }

    #[test]
    fn test_bolt_subtype_into_usize_returns_expected_code() {
        let code: usize = BoltSubType::Bolt.into();
        assert_eq!(code, 1);
    }

    #[test]
    fn test_spike_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(SpikeSubType::try_from(1usize).unwrap(), SpikeSubType::IronSpike);
    }

    #[test]
    fn test_spike_subtype_try_from_usize_rejects_unknown_values() {
        assert!(SpikeSubType::try_from(0usize).is_err());
        assert!(SpikeSubType::try_from(2usize).is_err());
    }

    #[test]
    fn test_spike_subtype_into_usize_returns_expected_code() {
        let code: usize = SpikeSubType::IronSpike.into();
        assert_eq!(code, 1);
    }
}
