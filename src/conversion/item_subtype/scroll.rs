use crate::model::item_subtype::Scroll1SubType;

pub fn from_usize(subtype: usize) -> Option<Scroll1SubType> {
    match subtype {
        277 => Some(Scroll1SubType::AggravateMonster),
        262 => Some(Scroll1SubType::Blessing),
        285 => Some(Scroll1SubType::CreateFood),
        260 => Some(Scroll1SubType::CurseArmor),
        258 => Some(Scroll1SubType::CurseWeapon),
        283 => Some(Scroll1SubType::Darkness),
        266 => Some(Scroll1SubType::Destruction),
        276 => Some(Scroll1SubType::DetectInvisible),
        286 => Some(Scroll1SubType::DispelUndead),
        280 => Some(Scroll1SubType::DoorCreation),
        274 => Some(Scroll1SubType::DoorStairLocation),
        259 => Some(Scroll1SubType::EnchantArmor),
        // 258 => Some(Scroll1SubType::EnchantWeapon), TODO UNREACHABLE
        257 => Some(Scroll1SubType::EnchantWeaponToDam),
        // 257 => Some(Scroll1SubType::EnchantWeaponToHit), TODO UNREACHABLE
        268 => Some(Scroll1SubType::FeignDeath),
        282 => Some(Scroll1SubType::Genocide),
        263 => Some(Scroll1SubType::HolyChant),
        264 => Some(Scroll1SubType::HolyPrayer),
        // 260 => Some(Scroll1SubType::Identify), TODO UNREACHABLE
        // 262 => Some(Scroll1SubType::Light), TODO UNREACHABLE
        // 268 => Some(Scroll1SubType::MagicMapping), TODO UNREACHABLE
        269 => Some(Scroll1SubType::MakeMunchies),
        275 => Some(Scroll1SubType::MassGenocide),
        267 => Some(Scroll1SubType::MonsterConfusion),
        272 => Some(Scroll1SubType::ObjectDetection),
        // 264 => Some(Scroll1SubType::PhaseDoor), TODO UNREACHABLE
        284 => Some(Scroll1SubType::ProtectionFromEvil),
        281 => Some(Scroll1SubType::Recharging),
        261 => Some(Scroll1SubType::RemoveCurse),
        270 => Some(Scroll1SubType::RuneOfProtection),
        // 269 => Some(Scroll1SubType::SleepMonster), TODO UNREACHABLE
        // 263 => Some(Scroll1SubType::SummonMonster), TODO UNREACHABLE
        // 261 => Some(Scroll1SubType::SummonUndead), TODO UNREACHABLE
        // 266 => Some(Scroll1SubType::Teleport), TODO UNREACHABLE
        265 => Some(Scroll1SubType::TeleportLevel),
        278 => Some(Scroll1SubType::TrapCreation),
        273 => Some(Scroll1SubType::TrapDetection),
        279 => Some(Scroll1SubType::TrapDoorDestruction),
        271 => Some(Scroll1SubType::TreasureDetection),
        // 267 => Some(Scroll1SubType::Wishing), TODO UNREACHABLE
        // 265 => Some(Scroll1SubType::WordOfRecall), TODO UNREACHABLE
        _ => None,
    }
}

pub fn to_usize(subtype: Scroll1SubType) -> usize {
    match subtype {
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
