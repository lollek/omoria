use crate::model::item_subtype::MaulSubType;

pub fn from_usize(subtype: usize) -> Option<MaulSubType> {
    match subtype {
        2 => Some(MaulSubType::BallAndChain),
        6 => Some(MaulSubType::WoodenClub),
        7 => Some(MaulSubType::Flail),
        8 => Some(MaulSubType::GreatFlail),
        9 => Some(MaulSubType::MorningStar),
        10 => Some(MaulSubType::Mace),
        11 => Some(MaulSubType::WarHammer),
        12 => Some(MaulSubType::LeadFilledMace),
        13 => Some(MaulSubType::IronShodQuarterstaff),
        14 => Some(MaulSubType::OgreMaul),
        _ => None,
    }
}

pub fn to_usize(subtype: MaulSubType) -> usize {
    match subtype {
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
