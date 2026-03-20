use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RangedWeaponSubType {
    Shortbow,
    HuntersBow,
    CompositeBow,
    WarBow,
    DoubleBow,
    SiegeBow,
    WardedBow,
    SiegeCrossbow,
    Ballista,
    LightCrossbow,
    HeavyCrossbow,
    Sling,
}

impl From<RangedWeaponSubType> for usize {
    fn from(value: RangedWeaponSubType) -> usize {
        match value {
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
}

impl TryFrom<usize> for RangedWeaponSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(RangedWeaponSubType::Shortbow),
            2 => Ok(RangedWeaponSubType::HuntersBow),
            3 => Ok(RangedWeaponSubType::CompositeBow),
            4 => Ok(RangedWeaponSubType::WarBow),
            5 => Ok(RangedWeaponSubType::DoubleBow),
            6 => Ok(RangedWeaponSubType::SiegeBow),
            7 => Ok(RangedWeaponSubType::WardedBow),
            10 => Ok(RangedWeaponSubType::SiegeCrossbow),
            11 => Ok(RangedWeaponSubType::Ballista),
            12 => Ok(RangedWeaponSubType::LightCrossbow),
            13 => Ok(RangedWeaponSubType::HeavyCrossbow),
            20 => Ok(RangedWeaponSubType::Sling),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::RangedWeaponSubType;

    #[test]
    fn test_ranged_weapon_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            RangedWeaponSubType::try_from(1usize).unwrap(),
            RangedWeaponSubType::Shortbow
        );
    }

    #[test]
    fn test_ranged_weapon_subtype_try_from_usize_rejects_unknown_value() {
        assert!(RangedWeaponSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_ranged_weapon_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(RangedWeaponSubType::Sling), 20);
    }
}
