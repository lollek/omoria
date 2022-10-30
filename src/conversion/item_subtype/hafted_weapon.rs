use crate::model::item_subtype::HaftedWeaponSubType;

pub fn from_usize(subtype: usize) -> Option<HaftedWeaponSubType> {
    match subtype {
        1 => Some(HaftedWeaponSubType::Balestarius),
        3 => Some(HaftedWeaponSubType::BattleAxe),
        4 => Some(HaftedWeaponSubType::BroadAxe),
        5 => Some(HaftedWeaponSubType::HandAxe),
        6 => Some(HaftedWeaponSubType::WarAxe),
        7 => Some(HaftedWeaponSubType::LargeAxe),
        8 => Some(HaftedWeaponSubType::BeardedAxe),
        9 => Some(HaftedWeaponSubType::SilverEdgedAxe),
        10 => Some(HaftedWeaponSubType::ChampionAxe),
        _ => None,
    }
}

pub fn to_usize(subtype: HaftedWeaponSubType) -> usize {
    match subtype {
        HaftedWeaponSubType::Balestarius => 1,
        HaftedWeaponSubType::BattleAxe => 3,
        HaftedWeaponSubType::BroadAxe => 4,
        HaftedWeaponSubType::HandAxe => 5,
        HaftedWeaponSubType::WarAxe => 6,
        HaftedWeaponSubType::LargeAxe => 7,
        HaftedWeaponSubType::BeardedAxe => 8,
        HaftedWeaponSubType::SilverEdgedAxe => 9,
        HaftedWeaponSubType::ChampionAxe => 10,
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
