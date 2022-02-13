use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ScrollTemplate {
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

impl ScrollTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(ScrollTemplate::AggravateMonster),
            Box::new(ScrollTemplate::Blessing),
            Box::new(ScrollTemplate::CreateFood),
            Box::new(ScrollTemplate::CurseArmor),
            Box::new(ScrollTemplate::CurseWeapon),
            Box::new(ScrollTemplate::Darkness),
            Box::new(ScrollTemplate::Destruction),
            Box::new(ScrollTemplate::DetectInvisible),
            Box::new(ScrollTemplate::DispelUndead),
            Box::new(ScrollTemplate::DoorCreation),
            Box::new(ScrollTemplate::DoorStairLocation),
            Box::new(ScrollTemplate::EnchantArmor),
            Box::new(ScrollTemplate::EnchantWeapon),
            Box::new(ScrollTemplate::EnchantWeaponToDam),
            Box::new(ScrollTemplate::EnchantWeaponToHit),
            Box::new(ScrollTemplate::FeignDeath),
            Box::new(ScrollTemplate::Genocide),
            Box::new(ScrollTemplate::HolyChant),
            Box::new(ScrollTemplate::HolyPrayer),
            Box::new(ScrollTemplate::Identify),
            Box::new(ScrollTemplate::Light),
            Box::new(ScrollTemplate::MagicMapping),
            Box::new(ScrollTemplate::MakeMunchies),
            Box::new(ScrollTemplate::MassGenocide),
            Box::new(ScrollTemplate::MonsterConfusion),
            Box::new(ScrollTemplate::ObjectDetection),
            Box::new(ScrollTemplate::PhaseDoor),
            Box::new(ScrollTemplate::ProtectionFromEvil),
            Box::new(ScrollTemplate::Recharging),
            Box::new(ScrollTemplate::RemoveCurse),
            Box::new(ScrollTemplate::RuneOfProtection),
            Box::new(ScrollTemplate::SleepMonster),
            Box::new(ScrollTemplate::SummonMonster),
            Box::new(ScrollTemplate::SummonUndead),
            Box::new(ScrollTemplate::Teleport),
            Box::new(ScrollTemplate::TeleportLevel),
            Box::new(ScrollTemplate::TrapCreation),
            Box::new(ScrollTemplate::TrapDetection),
            Box::new(ScrollTemplate::TrapDoorDestruction),
            Box::new(ScrollTemplate::TreasureDetection),
            Box::new(ScrollTemplate::Wishing),
            Box::new(ScrollTemplate::WordOfRecall),
            ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        ScrollTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            300 => Box::new(ScrollTemplate::AggravateMonster),
            301 => Box::new(ScrollTemplate::Blessing),
            302 => Box::new(ScrollTemplate::CreateFood),
            303 => Box::new(ScrollTemplate::CurseArmor),
            304 => Box::new(ScrollTemplate::CurseWeapon),
            305 => Box::new(ScrollTemplate::Darkness),
            306 => Box::new(ScrollTemplate::Destruction),
            307 => Box::new(ScrollTemplate::DetectInvisible),
            308 => Box::new(ScrollTemplate::DispelUndead),
            309 => Box::new(ScrollTemplate::DoorCreation),
            310 => Box::new(ScrollTemplate::DoorStairLocation),
            311 => Box::new(ScrollTemplate::EnchantArmor),
            312 => Box::new(ScrollTemplate::EnchantWeapon),
            313 => Box::new(ScrollTemplate::EnchantWeaponToDam),
            314 => Box::new(ScrollTemplate::EnchantWeaponToHit),
            315 => Box::new(ScrollTemplate::FeignDeath),
            316 => Box::new(ScrollTemplate::Genocide),
            317 => Box::new(ScrollTemplate::HolyChant),
            318 => Box::new(ScrollTemplate::HolyPrayer),
            319 => Box::new(ScrollTemplate::Identify),
            320 => Box::new(ScrollTemplate::Light),
            321 => Box::new(ScrollTemplate::MagicMapping),
            322 => Box::new(ScrollTemplate::MakeMunchies),
            323 => Box::new(ScrollTemplate::MassGenocide),
            324 => Box::new(ScrollTemplate::MonsterConfusion),
            325 => Box::new(ScrollTemplate::ObjectDetection),
            326 => Box::new(ScrollTemplate::PhaseDoor),
            327 => Box::new(ScrollTemplate::ProtectionFromEvil),
            328 => Box::new(ScrollTemplate::Recharging),
            329 => Box::new(ScrollTemplate::RemoveCurse),
            330 => Box::new(ScrollTemplate::RuneOfProtection),
            331 => Box::new(ScrollTemplate::SleepMonster),
            332 => Box::new(ScrollTemplate::SummonMonster),
            333 => Box::new(ScrollTemplate::SummonUndead),
            334 => Box::new(ScrollTemplate::Teleport),
            335 => Box::new(ScrollTemplate::TeleportLevel),
            336 => Box::new(ScrollTemplate::TrapCreation),
            337 => Box::new(ScrollTemplate::TrapDetection),
            338 => Box::new(ScrollTemplate::TrapDoorDestruction),
            339 => Box::new(ScrollTemplate::TreasureDetection),
            340 => Box::new(ScrollTemplate::Wishing),
            341 => Box::new(ScrollTemplate::WordOfRecall),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for ScrollTemplate {

    fn name(&self) -> &str {
        match self {
            ScrollTemplate::AggravateMonster => "& Scroll~| of Trap/Door Destruction",
            ScrollTemplate::Blessing => "& Scroll~| of Blessing",
            ScrollTemplate::CreateFood => "& Scroll~| of Create Food",
            ScrollTemplate::CurseArmor => "& Scroll~| of Curse Armor",
            ScrollTemplate::CurseWeapon => "& Scroll~| of Curse Weapon",
            ScrollTemplate::Darkness => "& Scroll~| of Darkness",
            ScrollTemplate::Destruction => "& Scroll~| of Destruction",
            ScrollTemplate::DetectInvisible => "& Scroll~| of Detect Invisible",
            ScrollTemplate::DispelUndead => "& Scroll~| of Dispel Undead",
            ScrollTemplate::DoorCreation => "& Scroll~| of Door Creation",
            ScrollTemplate::DoorStairLocation => "& Scroll~| of Door/Stair Location",
            ScrollTemplate::EnchantArmor => "& Scroll~| of Enchant Armor",
            ScrollTemplate::EnchantWeapon => "& Scroll~| of Enchant Weapon",
            ScrollTemplate::EnchantWeaponToDam => "& Scroll~| of Enchant Weapon To Dam",
            ScrollTemplate::EnchantWeaponToHit => "& Scroll~| of Enchant Weapon To Hit",
            ScrollTemplate::FeignDeath => "& Scroll~| of Feign Death",
            ScrollTemplate::Genocide => "G& Scroll~| of enocide",
            ScrollTemplate::HolyChant => "& Scroll~| of Holy Chant",
            ScrollTemplate::HolyPrayer => "& Scroll~| of Holy Prayer",
            ScrollTemplate::Identify => "& Scroll~| of Identify",
            ScrollTemplate::Light => "& Scroll~| of Light",
            ScrollTemplate::MagicMapping => "& Scroll~| of Magic Mapping",
            ScrollTemplate::MakeMunchies => "& Scroll~| of Make Munchies",
            ScrollTemplate::MassGenocide => "& Scroll~| of Mass Genocide",
            ScrollTemplate::MonsterConfusion => "& Scroll~| of Monster Confusion",
            ScrollTemplate::ObjectDetection => "& Scroll~| of Object Detection",
            ScrollTemplate::PhaseDoor => "& Scroll~| of Phase Door",
            ScrollTemplate::ProtectionFromEvil => "& Scroll~| of Protection from Evil",
            ScrollTemplate::Recharging => "& Scroll~| of Recharging",
            ScrollTemplate::RemoveCurse => "& Scroll~| of Remove Curse",
            ScrollTemplate::RuneOfProtection => "& Scroll~| of Rune of Protection",
            ScrollTemplate::SleepMonster => "& Scroll~| of Sleep Monster",
            ScrollTemplate::SummonMonster => "& Scroll~| of Summon Monster",
            ScrollTemplate::SummonUndead => "& Scroll~| of Summon Undead",
            ScrollTemplate::Teleport => "& Scroll~| of Teleport",
            ScrollTemplate::TeleportLevel => "& Scroll~| of Teleport Level",
            ScrollTemplate::TrapCreation => "& Scroll~| of Trap Creation",
            ScrollTemplate::TrapDetection => "& Scroll~| of Trap Detection",
            ScrollTemplate::TrapDoorDestruction => "& Scroll~| of Trap/Door Destruction",
            ScrollTemplate::TreasureDetection => "& Scroll~| of Treasure Detection",
            ScrollTemplate::Wishing => "& Scroll~| of Wishing",
            ScrollTemplate::WordOfRecall => "& Scroll~| of Word of Recall",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Scroll }

    fn flags1(&self) -> u64 {
        match self {
            ScrollTemplate::AggravateMonster => 0x00000000,
            ScrollTemplate::Blessing => 0x00000010,
            ScrollTemplate::CreateFood => 0x00000000,
            ScrollTemplate::CurseArmor => 0x00000004,
            ScrollTemplate::CurseWeapon => 0x00000001,
            ScrollTemplate::Darkness => 0x00000000,
            ScrollTemplate::Destruction => 0x00000100,
            ScrollTemplate::DetectInvisible => 0x00000000,
            ScrollTemplate::DispelUndead => 0x00000000,
            ScrollTemplate::DoorCreation => 0x00000000,
            ScrollTemplate::DoorStairLocation => 0x00000000,
            ScrollTemplate::EnchantArmor => 0x00000000,
            ScrollTemplate::EnchantWeapon => 0x00000000,
            ScrollTemplate::EnchantWeaponToDam => 0x00000000,
            ScrollTemplate::EnchantWeaponToHit => 0x00000000,
            ScrollTemplate::FeignDeath => 0x00000400,
            ScrollTemplate::Genocide => 0x00000000,
            ScrollTemplate::HolyChant => 0x00000020,
            ScrollTemplate::HolyPrayer => 0x00000040,
            ScrollTemplate::Identify => 0x00000000,
            ScrollTemplate::Light => 0x00000000,
            ScrollTemplate::MagicMapping => 0x00000000,
            ScrollTemplate::MakeMunchies => 0x00000800,
            ScrollTemplate::MassGenocide => 0x00000000,
            ScrollTemplate::MonsterConfusion => 0x00000000,
            ScrollTemplate::ObjectDetection => 0x00000000,
            ScrollTemplate::PhaseDoor => 0x00000000,
            ScrollTemplate::ProtectionFromEvil => 0x00000000,
            ScrollTemplate::Recharging => 0x00000000,
            ScrollTemplate::RemoveCurse => 0x00000000,
            ScrollTemplate::RuneOfProtection => 0x00000000,
            ScrollTemplate::SleepMonster => 0x00000000,
            ScrollTemplate::SummonMonster => 0x00000000,
            ScrollTemplate::SummonUndead => 0x00000008,
            ScrollTemplate::Teleport => 0x00000000,
            ScrollTemplate::TeleportLevel => 0x00000000,
            ScrollTemplate::TrapCreation => 0x00000000,
            ScrollTemplate::TrapDetection => 0x00000000,
            ScrollTemplate::TrapDoorDestruction => 0x00000000,
            ScrollTemplate::TreasureDetection => 0x00000000,
            ScrollTemplate::Wishing => 0x00000200,
            ScrollTemplate::WordOfRecall => 0x00000080,
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            ScrollTemplate::AggravateMonster => 0x00100000,
            ScrollTemplate::Blessing => 0x00000000,
            ScrollTemplate::CreateFood => 0x10000000,
            ScrollTemplate::CurseArmor => 0x00000000,
            ScrollTemplate::CurseWeapon => 0x00000000,
            ScrollTemplate::Darkness => 0x04000000,
            ScrollTemplate::Destruction => 0x00000000,
            ScrollTemplate::DetectInvisible => 0x00080000,
            ScrollTemplate::DispelUndead => 0x20000000,
            ScrollTemplate::DoorCreation => 0x00800000,
            ScrollTemplate::DoorStairLocation => 0x00020000,
            ScrollTemplate::EnchantArmor => 0x00000004,
            ScrollTemplate::EnchantWeapon => 0x00000002,
            ScrollTemplate::EnchantWeaponToDam => 0x00000001,
            ScrollTemplate::EnchantWeaponToHit => 0x00000000,
            ScrollTemplate::FeignDeath => 0x00000000,
            ScrollTemplate::Genocide => 0x02000000,
            ScrollTemplate::HolyChant => 0x00000000,
            ScrollTemplate::HolyPrayer => 0x00000000,
            ScrollTemplate::Identify => 0x00000008,
            ScrollTemplate::Light => 0x00000020,
            ScrollTemplate::MagicMapping => 0x00000800,
            ScrollTemplate::MakeMunchies => 0x00000000,
            ScrollTemplate::MassGenocide => 0x00040000,
            ScrollTemplate::MonsterConfusion => 0x00000400,
            ScrollTemplate::ObjectDetection => 0x00008000,
            ScrollTemplate::PhaseDoor => 0x00000080,
            ScrollTemplate::ProtectionFromEvil => 0x08000000,
            ScrollTemplate::Recharging => 0x01000000,
            ScrollTemplate::RemoveCurse => 0x00000010,
            ScrollTemplate::RuneOfProtection => 0x00002000,
            ScrollTemplate::SleepMonster => 0x00001000,
            ScrollTemplate::SummonMonster => 0x00000040,
            ScrollTemplate::SummonUndead => 0x00000000,
            ScrollTemplate::Teleport => 0x00000200,
            ScrollTemplate::TeleportLevel => 0x00000100,
            ScrollTemplate::TrapCreation => 0x00200000,
            ScrollTemplate::TrapDetection => 0x00010000,
            ScrollTemplate::TrapDoorDestruction => 0x00400000,
            ScrollTemplate::TreasureDetection => 0x00004000,
            ScrollTemplate::Wishing => 0x00000000,
            ScrollTemplate::WordOfRecall => 0x00000000,
        }
    }

    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            ScrollTemplate::AggravateMonster => 0,
            ScrollTemplate::Blessing => 15,
            ScrollTemplate::CreateFood => 15,
            ScrollTemplate::CurseArmor => 0,
            ScrollTemplate::CurseWeapon => 0,
            ScrollTemplate::Darkness => 0,
            ScrollTemplate::Destruction => 250,
            ScrollTemplate::DetectInvisible => 15,
            ScrollTemplate::DispelUndead => 200,
            ScrollTemplate::DoorCreation => 100,
            ScrollTemplate::DoorStairLocation => 35,
            ScrollTemplate::EnchantArmor => 125,
            ScrollTemplate::EnchantWeapon => 125,
            ScrollTemplate::EnchantWeaponToDam => 125,
            ScrollTemplate::EnchantWeaponToHit => 500,
            ScrollTemplate::FeignDeath => 0,
            ScrollTemplate::Genocide => 750,
            ScrollTemplate::HolyChant => 40,
            ScrollTemplate::HolyPrayer => 80,
            ScrollTemplate::Identify => 50,
            ScrollTemplate::Light => 15,
            ScrollTemplate::MagicMapping => 40,
            ScrollTemplate::MakeMunchies => 150,
            ScrollTemplate::MassGenocide => 1000,
            ScrollTemplate::MonsterConfusion => 30,
            ScrollTemplate::ObjectDetection => 15,
            ScrollTemplate::PhaseDoor => 15,
            ScrollTemplate::ProtectionFromEvil => 50,
            ScrollTemplate::Recharging => 200,
            ScrollTemplate::RemoveCurse => 100,
            ScrollTemplate::RuneOfProtection => 500,
            ScrollTemplate::SleepMonster => 35,
            ScrollTemplate::SummonMonster => 0,
            ScrollTemplate::SummonUndead => 0,
            ScrollTemplate::Teleport => 50,
            ScrollTemplate::TeleportLevel => 40,
            ScrollTemplate::TrapCreation => 0,
            ScrollTemplate::TrapDetection => 35,
            ScrollTemplate::TrapDoorDestruction => 50,
            ScrollTemplate::TreasureDetection => 15,
            ScrollTemplate::Wishing => 15000,
            ScrollTemplate::WordOfRecall => 150,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            ScrollTemplate::AggravateMonster => 300,
            ScrollTemplate::Blessing => 301,
            ScrollTemplate::CreateFood => 302,
            ScrollTemplate::CurseArmor => 303,
            ScrollTemplate::CurseWeapon => 304,
            ScrollTemplate::Darkness => 305,
            ScrollTemplate::Destruction => 306,
            ScrollTemplate::DetectInvisible => 307,
            ScrollTemplate::DispelUndead => 308,
            ScrollTemplate::DoorCreation => 309,
            ScrollTemplate::DoorStairLocation => 310,
            ScrollTemplate::EnchantArmor => 311,
            ScrollTemplate::EnchantWeapon => 312,
            ScrollTemplate::EnchantWeaponToDam => 313,
            ScrollTemplate::EnchantWeaponToHit => 314,
            ScrollTemplate::FeignDeath => 315,
            ScrollTemplate::Genocide => 316,
            ScrollTemplate::HolyChant => 317,
            ScrollTemplate::HolyPrayer => 318,
            ScrollTemplate::Identify => 319,
            ScrollTemplate::Light => 320,
            ScrollTemplate::MagicMapping => 321,
            ScrollTemplate::MakeMunchies => 322,
            ScrollTemplate::MassGenocide => 323,
            ScrollTemplate::MonsterConfusion => 324,
            ScrollTemplate::ObjectDetection => 325,
            ScrollTemplate::PhaseDoor => 326,
            ScrollTemplate::ProtectionFromEvil => 327,
            ScrollTemplate::Recharging => 328,
            ScrollTemplate::RemoveCurse => 329,
            ScrollTemplate::RuneOfProtection => 330,
            ScrollTemplate::SleepMonster => 331,
            ScrollTemplate::SummonMonster => 332,
            ScrollTemplate::SummonUndead => 333,
            ScrollTemplate::Teleport => 334,
            ScrollTemplate::TeleportLevel => 335,
            ScrollTemplate::TrapCreation => 336,
            ScrollTemplate::TrapDetection => 337,
            ScrollTemplate::TrapDoorDestruction => 338,
            ScrollTemplate::TreasureDetection => 339,
            ScrollTemplate::Wishing => 340,
            ScrollTemplate::WordOfRecall => 341,
        }
    }

    fn weight(&self) -> u16 { 5 }
    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "0d0" }

    fn item_level(&self) -> u8 {
        match self {
            ScrollTemplate::Light => 0,
            ScrollTemplate::ObjectDetection => 0,
            ScrollTemplate::TreasureDetection => 0,
            ScrollTemplate::Blessing => 1,
            ScrollTemplate::Darkness => 1,
            ScrollTemplate::DetectInvisible => 1,
            ScrollTemplate::Identify => 1,
            ScrollTemplate::PhaseDoor => 1,
            ScrollTemplate::SummonMonster => 1,
            ScrollTemplate::AggravateMonster => 5,
            ScrollTemplate::MagicMapping => 5,
            ScrollTemplate::MonsterConfusion => 5,
            ScrollTemplate::SleepMonster => 5,
            ScrollTemplate::TrapDetection => 5,
            ScrollTemplate::WordOfRecall => 5,
            ScrollTemplate::RemoveCurse => 7,
            ScrollTemplate::CreateFood => 10,
            ScrollTemplate::DoorStairLocation => 10,
            ScrollTemplate::FeignDeath => 10,
            ScrollTemplate::Teleport => 10,
            ScrollTemplate::DoorCreation => 12,
            ScrollTemplate::EnchantArmor => 12,
            ScrollTemplate::EnchantWeaponToDam => 12,
            ScrollTemplate::EnchantWeaponToHit => 12,
            ScrollTemplate::HolyChant => 12,
            ScrollTemplate::TrapCreation => 12,
            ScrollTemplate::TrapDoorDestruction => 12,
            ScrollTemplate::SummonUndead => 15,
            ScrollTemplate::TeleportLevel => 15,
            ScrollTemplate::HolyPrayer => 25,
            ScrollTemplate::MakeMunchies => 25,
            ScrollTemplate::ProtectionFromEvil => 30,
            ScrollTemplate::Genocide => 30,
            ScrollTemplate::Destruction => 40,
            ScrollTemplate::DispelUndead => 40,
            ScrollTemplate::Recharging => 40,
            ScrollTemplate::CurseArmor => 50,
            ScrollTemplate::CurseWeapon => 50,
            ScrollTemplate::EnchantWeapon => 50,
            ScrollTemplate::MassGenocide => 50,
            ScrollTemplate::RuneOfProtection => 50,
            ScrollTemplate::Wishing => 50,
        }
    }

    fn is_identified(&self) -> bool { false }
}
