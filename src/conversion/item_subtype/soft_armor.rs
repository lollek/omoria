use crate::model::item_subtype::SoftArmorSubType;

pub fn from_usize(subtype: usize) -> Option<SoftArmorSubType> {
    match subtype {
        11 => Some(SoftArmorSubType::CoolSetOfThreads),
        15 => Some(SoftArmorSubType::DemonhideArmor),
        14 => Some(SoftArmorSubType::DuskShroud),
        13 => Some(SoftArmorSubType::ElvenChainMail),
        12 => Some(SoftArmorSubType::FilthyNagaHideArmor),
        99 => Some(SoftArmorSubType::FilthyRags),
        4 => Some(SoftArmorSubType::HardLeatherArmor),
        8 => Some(SoftArmorSubType::HardLeatherRingMail),
        5 => Some(SoftArmorSubType::HardStuddedLeather),
        9 => Some(SoftArmorSubType::LeatherScaleMail),
        1 => Some(SoftArmorSubType::Robe),
        2 => Some(SoftArmorSubType::SoftLeatherArmor),
        7 => Some(SoftArmorSubType::SoftLeatherRingMail),
        3 => Some(SoftArmorSubType::SoftStuddedLeather),
        6 => Some(SoftArmorSubType::WovenCordArmor),
        16 => Some(SoftArmorSubType::WyrmhideArmor),
        10 => Some(SoftArmorSubType::LeatherBrigantineArmor),
        _ => None,
    }
}

pub fn to_usize(subtype: SoftArmorSubType) -> usize {
    match subtype {
        SoftArmorSubType::CoolSetOfThreads => 11,
        SoftArmorSubType::DemonhideArmor => 15,
        SoftArmorSubType::DuskShroud => 14,
        SoftArmorSubType::ElvenChainMail => 13,
        SoftArmorSubType::FilthyNagaHideArmor => 12,
        SoftArmorSubType::FilthyRags => 99,
        SoftArmorSubType::HardLeatherArmor => 4,
        SoftArmorSubType::HardLeatherRingMail => 8,
        SoftArmorSubType::HardStuddedLeather => 5,
        SoftArmorSubType::LeatherScaleMail => 9,
        SoftArmorSubType::Robe => 1,
        SoftArmorSubType::SoftLeatherArmor => 2,
        SoftArmorSubType::SoftLeatherRingMail => 7,
        SoftArmorSubType::SoftStuddedLeather => 3,
        SoftArmorSubType::WovenCordArmor => 6,
        SoftArmorSubType::WyrmhideArmor => 16,
        SoftArmorSubType::LeatherBrigantineArmor => 10,
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
