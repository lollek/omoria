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

// Code assignment scheme:
//   flags1 scrolls: 256 + (bit position + 1) → 257..=286
//   flags2 scrolls: 287 + flags2 bit position → 287..=298
//   EnchantWeaponToHit (no flags): 299

impl From<Scroll1SubType> for usize {
    fn from(value: Scroll1SubType) -> usize {
        match value {
            // flags1 scrolls (bit 0–29 → codes 257–286)
            Scroll1SubType::EnchantWeaponToDam => 257, // flags1 bit 0
            Scroll1SubType::EnchantWeapon => 258,      // flags1 bit 1
            Scroll1SubType::EnchantArmor => 259,       // flags1 bit 2
            Scroll1SubType::Identify => 260,           // flags1 bit 3
            Scroll1SubType::RemoveCurse => 261,        // flags1 bit 4
            Scroll1SubType::Light => 262,              // flags1 bit 5
            Scroll1SubType::SummonMonster => 263,      // flags1 bit 6
            Scroll1SubType::PhaseDoor => 264,          // flags1 bit 7
            Scroll1SubType::TeleportLevel => 265,      // flags1 bit 8
            Scroll1SubType::Teleport => 266,           // flags1 bit 9
            Scroll1SubType::MonsterConfusion => 267,   // flags1 bit 10
            Scroll1SubType::MagicMapping => 268,       // flags1 bit 11
            Scroll1SubType::SleepMonster => 269,       // flags1 bit 12
            Scroll1SubType::RuneOfProtection => 270,   // flags1 bit 13
            Scroll1SubType::TreasureDetection => 271,  // flags1 bit 14
            Scroll1SubType::ObjectDetection => 272,    // flags1 bit 15
            Scroll1SubType::TrapDetection => 273,      // flags1 bit 16
            Scroll1SubType::DoorStairLocation => 274,  // flags1 bit 17
            Scroll1SubType::MassGenocide => 275,       // flags1 bit 18
            Scroll1SubType::DetectInvisible => 276,    // flags1 bit 19
            Scroll1SubType::AggravateMonster => 277,   // flags1 bit 20
            Scroll1SubType::TrapCreation => 278,       // flags1 bit 21
            Scroll1SubType::TrapDoorDestruction => 279, // flags1 bit 22
            Scroll1SubType::DoorCreation => 280,       // flags1 bit 23
            Scroll1SubType::Recharging => 281,         // flags1 bit 24
            Scroll1SubType::Genocide => 282,           // flags1 bit 25
            Scroll1SubType::Darkness => 283,           // flags1 bit 26
            Scroll1SubType::ProtectionFromEvil => 284, // flags1 bit 27
            Scroll1SubType::CreateFood => 285,         // flags1 bit 28
            Scroll1SubType::DispelUndead => 286,       // flags1 bit 29
            // flags2 scrolls (bit N → code 287 + N)
            Scroll1SubType::CurseWeapon => 287,  // flags2 bit 0
            Scroll1SubType::CurseArmor => 289,   // flags2 bit 2
            Scroll1SubType::SummonUndead => 290, // flags2 bit 3
            Scroll1SubType::Blessing => 291,     // flags2 bit 4
            Scroll1SubType::HolyChant => 292,    // flags2 bit 5
            Scroll1SubType::HolyPrayer => 293,   // flags2 bit 6
            Scroll1SubType::WordOfRecall => 294, // flags2 bit 7
            Scroll1SubType::Destruction => 295,  // flags2 bit 8
            Scroll1SubType::Wishing => 296,      // flags2 bit 9
            Scroll1SubType::FeignDeath => 297,   // flags2 bit 10
            Scroll1SubType::MakeMunchies => 298, // flags2 bit 11
            // No flags
            Scroll1SubType::EnchantWeaponToHit => 299,
        }
    }
}

impl TryFrom<usize> for Scroll1SubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            // flags1 scrolls
            257 => Ok(Scroll1SubType::EnchantWeaponToDam),
            258 => Ok(Scroll1SubType::EnchantWeapon),
            259 => Ok(Scroll1SubType::EnchantArmor),
            260 => Ok(Scroll1SubType::Identify),
            261 => Ok(Scroll1SubType::RemoveCurse),
            262 => Ok(Scroll1SubType::Light),
            263 => Ok(Scroll1SubType::SummonMonster),
            264 => Ok(Scroll1SubType::PhaseDoor),
            265 => Ok(Scroll1SubType::TeleportLevel),
            266 => Ok(Scroll1SubType::Teleport),
            267 => Ok(Scroll1SubType::MonsterConfusion),
            268 => Ok(Scroll1SubType::MagicMapping),
            269 => Ok(Scroll1SubType::SleepMonster),
            270 => Ok(Scroll1SubType::RuneOfProtection),
            271 => Ok(Scroll1SubType::TreasureDetection),
            272 => Ok(Scroll1SubType::ObjectDetection),
            273 => Ok(Scroll1SubType::TrapDetection),
            274 => Ok(Scroll1SubType::DoorStairLocation),
            275 => Ok(Scroll1SubType::MassGenocide),
            276 => Ok(Scroll1SubType::DetectInvisible),
            277 => Ok(Scroll1SubType::AggravateMonster),
            278 => Ok(Scroll1SubType::TrapCreation),
            279 => Ok(Scroll1SubType::TrapDoorDestruction),
            280 => Ok(Scroll1SubType::DoorCreation),
            281 => Ok(Scroll1SubType::Recharging),
            282 => Ok(Scroll1SubType::Genocide),
            283 => Ok(Scroll1SubType::Darkness),
            284 => Ok(Scroll1SubType::ProtectionFromEvil),
            285 => Ok(Scroll1SubType::CreateFood),
            286 => Ok(Scroll1SubType::DispelUndead),
            // flags2 scrolls
            287 => Ok(Scroll1SubType::CurseWeapon),
            289 => Ok(Scroll1SubType::CurseArmor),
            290 => Ok(Scroll1SubType::SummonUndead),
            291 => Ok(Scroll1SubType::Blessing),
            292 => Ok(Scroll1SubType::HolyChant),
            293 => Ok(Scroll1SubType::HolyPrayer),
            294 => Ok(Scroll1SubType::WordOfRecall),
            295 => Ok(Scroll1SubType::Destruction),
            296 => Ok(Scroll1SubType::Wishing),
            297 => Ok(Scroll1SubType::FeignDeath),
            298 => Ok(Scroll1SubType::MakeMunchies),
            // No flags
            299 => Ok(Scroll1SubType::EnchantWeaponToHit),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::convert::TryFrom;

    use super::Scroll1SubType;

    const ALL_VARIANTS: [Scroll1SubType; 42] = [
        Scroll1SubType::AggravateMonster,
        Scroll1SubType::Blessing,
        Scroll1SubType::CreateFood,
        Scroll1SubType::CurseArmor,
        Scroll1SubType::CurseWeapon,
        Scroll1SubType::Darkness,
        Scroll1SubType::Destruction,
        Scroll1SubType::DetectInvisible,
        Scroll1SubType::DispelUndead,
        Scroll1SubType::DoorCreation,
        Scroll1SubType::DoorStairLocation,
        Scroll1SubType::EnchantArmor,
        Scroll1SubType::EnchantWeapon,
        Scroll1SubType::EnchantWeaponToDam,
        Scroll1SubType::EnchantWeaponToHit,
        Scroll1SubType::FeignDeath,
        Scroll1SubType::Genocide,
        Scroll1SubType::HolyChant,
        Scroll1SubType::HolyPrayer,
        Scroll1SubType::Identify,
        Scroll1SubType::Light,
        Scroll1SubType::MagicMapping,
        Scroll1SubType::MakeMunchies,
        Scroll1SubType::MassGenocide,
        Scroll1SubType::MonsterConfusion,
        Scroll1SubType::ObjectDetection,
        Scroll1SubType::PhaseDoor,
        Scroll1SubType::ProtectionFromEvil,
        Scroll1SubType::Recharging,
        Scroll1SubType::RemoveCurse,
        Scroll1SubType::RuneOfProtection,
        Scroll1SubType::SleepMonster,
        Scroll1SubType::SummonMonster,
        Scroll1SubType::SummonUndead,
        Scroll1SubType::Teleport,
        Scroll1SubType::TeleportLevel,
        Scroll1SubType::TrapCreation,
        Scroll1SubType::TrapDetection,
        Scroll1SubType::TrapDoorDestruction,
        Scroll1SubType::TreasureDetection,
        Scroll1SubType::Wishing,
        Scroll1SubType::WordOfRecall,
    ];

    #[test]
    fn all_codes_are_unique() {
        let mut seen = HashSet::new();
        for variant in &ALL_VARIANTS {
            let code = usize::from(*variant);
            assert!(
                seen.insert(code),
                "Duplicate code {} for {:?}",
                code,
                variant
            );
        }
    }

    #[test]
    fn all_variants_round_trip_through_usize() {
        for variant in &ALL_VARIANTS {
            let code = usize::from(*variant);
            let back = Scroll1SubType::try_from(code).unwrap_or_else(|_| {
                panic!("TryFrom failed for code {} (variant {:?})", code, variant)
            });
            assert_eq!(
                *variant, back,
                "Round-trip failed: {variant:?} -> {code} -> {back:?}"
            );
        }
    }

    #[test]
    fn try_from_rejects_unknown_value() {
        assert!(Scroll1SubType::try_from(0usize).is_err());
        assert!(Scroll1SubType::try_from(256usize).is_err());
        assert!(
            Scroll1SubType::try_from(288usize).is_err(),
            "288 is a gap (flags2 bit 1 unused)"
        );
        assert!(Scroll1SubType::try_from(300usize).is_err());
    }
}
