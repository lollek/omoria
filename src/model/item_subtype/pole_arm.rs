use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PoleArmSubType {
    AwlPike,
    BeakedAxe,
    Fauchard,
    Glaive,
    Halberd,
    LucerneHammer,
    Pike,
    Spike,
    Lance,
    Javelin,
    Naginata,
    WarScythe,
}

impl From<PoleArmSubType> for usize {
    fn from(value: PoleArmSubType) -> usize {
        match value {
            PoleArmSubType::AwlPike => 1,
            PoleArmSubType::BeakedAxe => 2,
            PoleArmSubType::Fauchard => 3,
            PoleArmSubType::Glaive => 4,
            PoleArmSubType::Halberd => 5,
            PoleArmSubType::LucerneHammer => 6,
            PoleArmSubType::Pike => 7,
            PoleArmSubType::Spike => 8,
            PoleArmSubType::Lance => 9,
            PoleArmSubType::Javelin => 10,
            PoleArmSubType::Naginata => 11,
            PoleArmSubType::WarScythe => 12,
        }
    }
}

impl TryFrom<usize> for PoleArmSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PoleArmSubType::AwlPike),
            2 => Ok(PoleArmSubType::BeakedAxe),
            3 => Ok(PoleArmSubType::Fauchard),
            4 => Ok(PoleArmSubType::Glaive),
            5 => Ok(PoleArmSubType::Halberd),
            6 => Ok(PoleArmSubType::LucerneHammer),
            7 => Ok(PoleArmSubType::Pike),
            8 => Ok(PoleArmSubType::Spike),
            9 => Ok(PoleArmSubType::Lance),
            10 => Ok(PoleArmSubType::Javelin),
            11 => Ok(PoleArmSubType::Naginata),
            12 => Ok(PoleArmSubType::WarScythe),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::PoleArmSubType;

    #[test]
    fn test_pole_arm_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            PoleArmSubType::try_from(1usize).unwrap(),
            PoleArmSubType::AwlPike
        );
    }

    #[test]
    fn test_pole_arm_subtype_try_from_usize_rejects_unknown_value() {
        assert!(PoleArmSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_pole_arm_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(PoleArmSubType::WarScythe), 12);
    }
}
