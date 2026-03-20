use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MiscObjectSubType {
    RatSkeleton,
    GiantCentipedeSkeleton,
    EmptyBottle,
    PotteryShard,
    HumanSkeleton,
    DwarfSkeleton,
    ElfSkeleton,
    GnomeSkeleton,
    BrokenTeeth,
    LargeBrokenBone,
    BrokenStick,
}

impl From<MiscObjectSubType> for usize {
    fn from(value: MiscObjectSubType) -> usize {
        match value {
            MiscObjectSubType::RatSkeleton => 1,
            MiscObjectSubType::GiantCentipedeSkeleton => 2,
            MiscObjectSubType::EmptyBottle => 4,
            MiscObjectSubType::PotteryShard => 5,
            MiscObjectSubType::HumanSkeleton => 7,
            MiscObjectSubType::DwarfSkeleton => 8,
            MiscObjectSubType::ElfSkeleton => 9,
            MiscObjectSubType::GnomeSkeleton => 10,
            MiscObjectSubType::BrokenTeeth => 11,
            MiscObjectSubType::LargeBrokenBone => 12,
            MiscObjectSubType::BrokenStick => 13,
        }
    }
}

impl TryFrom<usize> for MiscObjectSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MiscObjectSubType::RatSkeleton),
            2 => Ok(MiscObjectSubType::GiantCentipedeSkeleton),
            4 => Ok(MiscObjectSubType::EmptyBottle),
            5 => Ok(MiscObjectSubType::PotteryShard),
            7 => Ok(MiscObjectSubType::HumanSkeleton),
            8 => Ok(MiscObjectSubType::DwarfSkeleton),
            9 => Ok(MiscObjectSubType::ElfSkeleton),
            10 => Ok(MiscObjectSubType::GnomeSkeleton),
            11 => Ok(MiscObjectSubType::BrokenTeeth),
            12 => Ok(MiscObjectSubType::LargeBrokenBone),
            13 => Ok(MiscObjectSubType::BrokenStick),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::MiscObjectSubType;

    #[test]
    fn test_misc_object_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            MiscObjectSubType::try_from(1usize).unwrap(),
            MiscObjectSubType::RatSkeleton
        );
    }

    #[test]
    fn test_misc_object_subtype_try_from_usize_rejects_unknown_value() {
        assert!(MiscObjectSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_misc_object_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(MiscObjectSubType::BrokenStick), 13);
    }
}
