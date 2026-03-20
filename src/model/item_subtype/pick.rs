use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PickSubType {
    Pick,
    Shovel,
    OrcishPick1,
    OrcishPick2,
    DwarvenPick,
    GnomishShovel,
    DwarvenShovel,
}

impl From<PickSubType> for usize {
    fn from(value: PickSubType) -> usize {
        match value {
            PickSubType::Pick => 1,
            PickSubType::Shovel => 2,
            PickSubType::OrcishPick1 => 8,
            PickSubType::OrcishPick2 => 7,
            PickSubType::DwarvenPick => 3,
            PickSubType::GnomishShovel => 5,
            PickSubType::DwarvenShovel => 6,
        }
    }
}

impl TryFrom<usize> for PickSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PickSubType::Pick),
            2 => Ok(PickSubType::Shovel),
            8 => Ok(PickSubType::OrcishPick1),
            7 => Ok(PickSubType::OrcishPick2),
            3 => Ok(PickSubType::DwarvenPick),
            5 => Ok(PickSubType::GnomishShovel),
            6 => Ok(PickSubType::DwarvenShovel),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::PickSubType;

    #[test]
    fn test_pick_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(PickSubType::try_from(1usize).unwrap(), PickSubType::Pick);
    }

    #[test]
    fn test_pick_subtype_try_from_usize_rejects_unknown_value() {
        assert!(PickSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_pick_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(PickSubType::OrcishPick1), 8);
    }
}
