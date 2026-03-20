use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HaftedWeaponSubType {
    Balestarius,
    BattleAxe,
    BroadAxe,
    HandAxe,
    WarAxe,
    LargeAxe,
    BeardedAxe,
    SilverEdgedAxe,
    ChampionAxe,
}

impl From<HaftedWeaponSubType> for usize {
    fn from(value: HaftedWeaponSubType) -> usize {
        match value {
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
}

impl TryFrom<usize> for HaftedWeaponSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(HaftedWeaponSubType::Balestarius),
            3 => Ok(HaftedWeaponSubType::BattleAxe),
            4 => Ok(HaftedWeaponSubType::BroadAxe),
            5 => Ok(HaftedWeaponSubType::HandAxe),
            6 => Ok(HaftedWeaponSubType::WarAxe),
            7 => Ok(HaftedWeaponSubType::LargeAxe),
            8 => Ok(HaftedWeaponSubType::BeardedAxe),
            9 => Ok(HaftedWeaponSubType::SilverEdgedAxe),
            10 => Ok(HaftedWeaponSubType::ChampionAxe),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::HaftedWeaponSubType;

    #[test]
    fn test_hafted_weapon_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            HaftedWeaponSubType::try_from(1usize).unwrap(),
            HaftedWeaponSubType::Balestarius
        );
    }

    #[test]
    fn test_hafted_weapon_subtype_try_from_usize_rejects_unknown_value() {
        assert!(HaftedWeaponSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_hafted_weapon_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(HaftedWeaponSubType::ChampionAxe), 10);
    }
}
