use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum WearableGemTemplate {
    GemOfTeleportation,
    GemOfResistCold,
    GemOfResistAcid,
    GemOfSeeInvisible,
    GemOfStealth,
    GemOfSlowDigestion,
    GemOfProtectFire,
}

impl WearableGemTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(WearableGemTemplate::GemOfTeleportation),
            Box::new(WearableGemTemplate::GemOfResistCold),
            Box::new(WearableGemTemplate::GemOfResistAcid),
            Box::new(WearableGemTemplate::GemOfSeeInvisible),
            Box::new(WearableGemTemplate::GemOfStealth),
            Box::new(WearableGemTemplate::GemOfSlowDigestion),
            Box::new(WearableGemTemplate::GemOfProtectFire),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        WearableGemTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(WearableGemTemplate::GemOfTeleportation),
            2 => Box::new(WearableGemTemplate::GemOfResistCold),
            3 => Box::new(WearableGemTemplate::GemOfResistAcid),
            4 => Box::new(WearableGemTemplate::GemOfSeeInvisible),
            5 => Box::new(WearableGemTemplate::GemOfStealth),
            6 => Box::new(WearableGemTemplate::GemOfSlowDigestion),
            7 => Box::new(WearableGemTemplate::GemOfProtectFire),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for WearableGemTemplate {
    fn name(&self) -> &str {
        match self {
            WearableGemTemplate::GemOfTeleportation => "& Finely cut| of Teleportation^",
            WearableGemTemplate::GemOfResistCold => "& Finely cut| of Resist Cold^",
            WearableGemTemplate::GemOfResistAcid => "& Finely cut| of Resist Acid^",
            WearableGemTemplate::GemOfSeeInvisible => "& Finely cut| of See Invisible^",
            WearableGemTemplate::GemOfStealth => "& Finely cut| of Stealth^",
            WearableGemTemplate::GemOfSlowDigestion => "& Finely cut| of Slow Digestion^",
            WearableGemTemplate::GemOfProtectFire => "& Finely cut| of Lordly Protection (FIRE)^",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::WearableGem }
    fn flags1(&self) -> u64 { 0 }

    fn flags2(&self) -> u64 {
        match self {
            WearableGemTemplate::GemOfTeleportation => 0x00000400,
            WearableGemTemplate::GemOfResistCold => 0x00200000,
            WearableGemTemplate::GemOfResistAcid => 0x00100000,
            WearableGemTemplate::GemOfSeeInvisible => 0x01000000,
            WearableGemTemplate::GemOfStealth => 0x00000100,
            WearableGemTemplate::GemOfSlowDigestion => 0x00000800,
            WearableGemTemplate::GemOfProtectFire => 0x00000800,
        }
    }

    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            WearableGemTemplate::GemOfTeleportation => 300,
            WearableGemTemplate::GemOfResistCold => 250,
            WearableGemTemplate::GemOfResistAcid => 250,
            WearableGemTemplate::GemOfSeeInvisible => 350,
            WearableGemTemplate::GemOfStealth => 300,
            WearableGemTemplate::GemOfSlowDigestion => 200,
            WearableGemTemplate::GemOfProtectFire => 1200,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            WearableGemTemplate::GemOfTeleportation => 1,
            WearableGemTemplate::GemOfResistCold => 2,
            WearableGemTemplate::GemOfResistAcid => 3,
            WearableGemTemplate::GemOfSeeInvisible => 4,
            WearableGemTemplate::GemOfStealth => 5,
            WearableGemTemplate::GemOfSlowDigestion => 6,
            WearableGemTemplate::GemOfProtectFire => 7,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            WearableGemTemplate::GemOfTeleportation => 5,
            WearableGemTemplate::GemOfResistCold => 5,
            WearableGemTemplate::GemOfResistAcid => 5,
            WearableGemTemplate::GemOfSeeInvisible => 5,
            WearableGemTemplate::GemOfStealth => 5,
            WearableGemTemplate::GemOfSlowDigestion => 5,
            WearableGemTemplate::GemOfProtectFire => 5,
        }
    }

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }

    fn item_level(&self) -> u8 {
        match self {
            WearableGemTemplate::GemOfTeleportation => 5,
            WearableGemTemplate::GemOfResistCold => 14,
            WearableGemTemplate::GemOfResistAcid => 14,
            WearableGemTemplate::GemOfSeeInvisible => 40,
            WearableGemTemplate::GemOfStealth => 35,
            WearableGemTemplate::GemOfSlowDigestion => 14,
            WearableGemTemplate::GemOfProtectFire => 40,
        }
    }

    fn is_identified(&self) -> bool { false }
}

