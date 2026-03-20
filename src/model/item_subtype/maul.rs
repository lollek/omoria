use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaulSubType {
    BallAndChain,
    WoodenClub,
    Flail,
    GreatFlail,
    MorningStar,
    Mace,
    WarHammer,
    LeadFilledMace,
    IronShodQuarterstaff,
    OgreMaul,
}

impl From<MaulSubType> for usize {
    fn from(value: MaulSubType) -> usize {
        match value {
            MaulSubType::BallAndChain => 2,
            MaulSubType::WoodenClub => 6,
            MaulSubType::Flail => 7,
            MaulSubType::GreatFlail => 8,
            MaulSubType::MorningStar => 9,
            MaulSubType::Mace => 10,
            MaulSubType::WarHammer => 11,
            MaulSubType::LeadFilledMace => 12,
            MaulSubType::IronShodQuarterstaff => 13,
            MaulSubType::OgreMaul => 14,
        }
    }
}

impl TryFrom<usize> for MaulSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(MaulSubType::BallAndChain),
            6 => Ok(MaulSubType::WoodenClub),
            7 => Ok(MaulSubType::Flail),
            8 => Ok(MaulSubType::GreatFlail),
            9 => Ok(MaulSubType::MorningStar),
            10 => Ok(MaulSubType::Mace),
            11 => Ok(MaulSubType::WarHammer),
            12 => Ok(MaulSubType::LeadFilledMace),
            13 => Ok(MaulSubType::IronShodQuarterstaff),
            14 => Ok(MaulSubType::OgreMaul),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::MaulSubType;

    #[test]
    fn test_maul_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            MaulSubType::try_from(2usize).unwrap(),
            MaulSubType::BallAndChain
        );
    }

    #[test]
    fn test_maul_subtype_try_from_usize_rejects_unknown_value() {
        assert!(MaulSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_maul_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(MaulSubType::OgreMaul), 14);
    }
}
