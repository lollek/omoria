use crate::model::item_subtype::MiscObjectSubType;

pub fn from_usize(subtype: usize) -> Option<MiscObjectSubType> {
    match subtype {
        1 => Some(MiscObjectSubType::RatSkeleton),
        2 => Some(MiscObjectSubType::GiantCentipedeSkeleton),
        4 => Some(MiscObjectSubType::EmptyBottle),
        5 => Some(MiscObjectSubType::PotteryShard),
        7 => Some(MiscObjectSubType::HumanSkeleton),
        8 => Some(MiscObjectSubType::DwarfSkeleton),
        9 => Some(MiscObjectSubType::ElfSkeleton),
        10 => Some(MiscObjectSubType::GnomeSkeleton),
        11 => Some(MiscObjectSubType::BrokenTeeth),
        12 => Some(MiscObjectSubType::LargeBrokenBone),
        13 => Some(MiscObjectSubType::BrokenStick),
        _ => None,
    }
}

pub fn to_usize(light_source: MiscObjectSubType) -> usize {
    match light_source {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_conversion() {
        (0..1000).for_each(|i| {
            if let Some(subtype) = from_usize(i) {
                assert_eq!(i, to_usize(subtype));
            }
        })
    }
}
