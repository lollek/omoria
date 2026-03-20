use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scroll1SubType {
    AggravateMonster,
    Blessing,
    CreateFood,
    CurseArmor,
    CurseWeapon,
    Darkness,
    Destruction,
    DetectInvisible,
    DispelUndead,
    DoorCreation,
    DoorStairLocation,
    EnchantArmor,
    EnchantWeapon,
    EnchantWeaponToDam,
    EnchantWeaponToHit,
    FeignDeath,
    Genocide,
    HolyChant,
    HolyPrayer,
    Identify,
    Light,
    MagicMapping,
    MakeMunchies,
    MassGenocide,
    MonsterConfusion,
    ObjectDetection,
    PhaseDoor,
    ProtectionFromEvil,
    Recharging,
    RemoveCurse,
    RuneOfProtection,
    SleepMonster,
    SummonMonster,
    SummonUndead,
    Teleport,
    TeleportLevel,
    TrapCreation,
    TrapDetection,
    TrapDoorDestruction,
    TreasureDetection,
    Wishing,
    WordOfRecall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scroll2SubType {}

impl From<Scroll1SubType> for usize {
    fn from(value: Scroll1SubType) -> usize {
        match value {
            Scroll1SubType::AggravateMonster => 277,
            Scroll1SubType::Blessing => 262,
            Scroll1SubType::CreateFood => 285,
            Scroll1SubType::CurseArmor => 260,
            Scroll1SubType::CurseWeapon => 258,
            Scroll1SubType::Darkness => 283,
            Scroll1SubType::Destruction => 266,
            Scroll1SubType::DetectInvisible => 276,
            Scroll1SubType::DispelUndead => 286,
            Scroll1SubType::DoorCreation => 280,
            Scroll1SubType::DoorStairLocation => 274,
            Scroll1SubType::EnchantArmor => 259,
            Scroll1SubType::EnchantWeapon => 258,
            Scroll1SubType::EnchantWeaponToDam => 257,
            Scroll1SubType::EnchantWeaponToHit => 257,
            Scroll1SubType::FeignDeath => 268,
            Scroll1SubType::Genocide => 282,
            Scroll1SubType::HolyChant => 263,
            Scroll1SubType::HolyPrayer => 264,
            Scroll1SubType::Identify => 260,
            Scroll1SubType::Light => 262,
            Scroll1SubType::MagicMapping => 268,
            Scroll1SubType::MakeMunchies => 269,
            Scroll1SubType::MassGenocide => 275,
            Scroll1SubType::MonsterConfusion => 267,
            Scroll1SubType::ObjectDetection => 272,
            Scroll1SubType::PhaseDoor => 264,
            Scroll1SubType::ProtectionFromEvil => 284,
            Scroll1SubType::Recharging => 281,
            Scroll1SubType::RemoveCurse => 261,
            Scroll1SubType::RuneOfProtection => 270,
            Scroll1SubType::SleepMonster => 269,
            Scroll1SubType::SummonMonster => 263,
            Scroll1SubType::SummonUndead => 261,
            Scroll1SubType::Teleport => 266,
            Scroll1SubType::TeleportLevel => 265,
            Scroll1SubType::TrapCreation => 278,
            Scroll1SubType::TrapDetection => 273,
            Scroll1SubType::TrapDoorDestruction => 279,
            Scroll1SubType::TreasureDetection => 271,
            Scroll1SubType::Wishing => 267,
            Scroll1SubType::WordOfRecall => 265,
        }
    }
}

impl TryFrom<usize> for Scroll1SubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            277 => Ok(Scroll1SubType::AggravateMonster),
            262 => Ok(Scroll1SubType::Blessing),
            285 => Ok(Scroll1SubType::CreateFood),
            260 => Ok(Scroll1SubType::CurseArmor),
            258 => Ok(Scroll1SubType::CurseWeapon),
            283 => Ok(Scroll1SubType::Darkness),
            266 => Ok(Scroll1SubType::Destruction),
            276 => Ok(Scroll1SubType::DetectInvisible),
            286 => Ok(Scroll1SubType::DispelUndead),
            280 => Ok(Scroll1SubType::DoorCreation),
            274 => Ok(Scroll1SubType::DoorStairLocation),
            259 => Ok(Scroll1SubType::EnchantArmor),
            268 => Ok(Scroll1SubType::FeignDeath),
            282 => Ok(Scroll1SubType::Genocide),
            263 => Ok(Scroll1SubType::HolyChant),
            264 => Ok(Scroll1SubType::HolyPrayer),
            269 => Ok(Scroll1SubType::MakeMunchies),
            275 => Ok(Scroll1SubType::MassGenocide),
            267 => Ok(Scroll1SubType::MonsterConfusion),
            272 => Ok(Scroll1SubType::ObjectDetection),
            284 => Ok(Scroll1SubType::ProtectionFromEvil),
            281 => Ok(Scroll1SubType::Recharging),
            261 => Ok(Scroll1SubType::RemoveCurse),
            270 => Ok(Scroll1SubType::RuneOfProtection),
            265 => Ok(Scroll1SubType::TeleportLevel),
            278 => Ok(Scroll1SubType::TrapCreation),
            273 => Ok(Scroll1SubType::TrapDetection),
            279 => Ok(Scroll1SubType::TrapDoorDestruction),
            271 => Ok(Scroll1SubType::TreasureDetection),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::Scroll1SubType;

    #[test]
    fn test_scroll1_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            Scroll1SubType::try_from(277usize).unwrap(),
            Scroll1SubType::AggravateMonster
        );
    }

    #[test]
    fn test_scroll1_subtype_try_from_usize_rejects_unknown_value() {
        assert!(Scroll1SubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_scroll1_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(Scroll1SubType::WordOfRecall), 265);
    }
}
