use crate::model::item_subtype::RangedWeaponSubType;

pub fn from_usize(subtype: usize) -> Option<RangedWeaponSubType> {
    match subtype {
        1 => Some(RangedWeaponSubType::Shortbow),
        2 => Some(RangedWeaponSubType::HuntersBow),
        3 => Some(RangedWeaponSubType::CompositeBow),
        4 => Some(RangedWeaponSubType::WarBow),
        5 => Some(RangedWeaponSubType::DoubleBow),
        6 => Some(RangedWeaponSubType::SiegeBow),
        7 => Some(RangedWeaponSubType::WardedBow),
        10 => Some(RangedWeaponSubType::SiegeCrossbow),
        11 => Some(RangedWeaponSubType::Ballista),
        12 => Some(RangedWeaponSubType::LightCrossbow),
        13 => Some(RangedWeaponSubType::HeavyCrossbow),
        20 => Some(RangedWeaponSubType::Sling),
        _ => None,
    }
}

pub fn to_usize(subtype: RangedWeaponSubType) -> usize {
    match subtype {
        RangedWeaponSubType::Shortbow => 1,
        RangedWeaponSubType::HuntersBow => 2,
        RangedWeaponSubType::CompositeBow => 3,
        RangedWeaponSubType::WarBow => 4,
        RangedWeaponSubType::DoubleBow => 5,
        RangedWeaponSubType::SiegeBow => 6,
        RangedWeaponSubType::WardedBow => 7,
        RangedWeaponSubType::SiegeCrossbow => 10,
        RangedWeaponSubType::Ballista => 11,
        RangedWeaponSubType::LightCrossbow => 12,
        RangedWeaponSubType::HeavyCrossbow => 13,
        RangedWeaponSubType::Sling => 20,
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
