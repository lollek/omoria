use crate::model::item_subtype::HardArmorSubType;

pub fn from_usize(subtype: usize) -> Option<HardArmorSubType> {
    match subtype {
        5 => Some(HardArmorSubType::AugmentedChainMail),
        6 => Some(HardArmorSubType::BarChainMail),
        13 => Some(HardArmorSubType::BronzePlateMail),
        2 => Some(HardArmorSubType::ChainMail),
        4 => Some(HardArmorSubType::DoubleChainMail),
        11 => Some(HardArmorSubType::FullPlateArmor),
        12 => Some(HardArmorSubType::LacqueredPlate),
        8 => Some(HardArmorSubType::LaminatedArmor),
        7 => Some(HardArmorSubType::MetalBrigandineArmor),
        10 => Some(HardArmorSubType::MetalLamellarArmor),
        1 => Some(HardArmorSubType::MetalScaleMail),
        15 => Some(HardArmorSubType::MithrilChainMail),
        16 => Some(HardArmorSubType::MithrilPlateArmor),
        9 => Some(HardArmorSubType::PartialPlateArmor),
        3 => Some(HardArmorSubType::RustyChainMail),
        14 => Some(HardArmorSubType::StonePlateArmor),
        _ => None,
    }
}

pub fn to_usize(subtype: HardArmorSubType) -> usize {
    match subtype {
        HardArmorSubType::AugmentedChainMail => 5,
        HardArmorSubType::BarChainMail => 6,
        HardArmorSubType::BronzePlateMail => 13,
        HardArmorSubType::ChainMail => 2,
        HardArmorSubType::DoubleChainMail => 4,
        HardArmorSubType::FullPlateArmor => 11,
        HardArmorSubType::LacqueredPlate => 12,
        HardArmorSubType::LaminatedArmor => 8,
        HardArmorSubType::MetalBrigandineArmor => 7,
        HardArmorSubType::MetalLamellarArmor => 10,
        HardArmorSubType::MetalScaleMail => 1,
        HardArmorSubType::MithrilChainMail => 15,
        HardArmorSubType::MithrilPlateArmor => 16,
        HardArmorSubType::PartialPlateArmor => 9,
        HardArmorSubType::RustyChainMail => 3,
        HardArmorSubType::StonePlateArmor => 14,
    }
}

#[cfg(test)]
mod test {
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
