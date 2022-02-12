use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AmuletTemplate {
    AmuletOfAdornment1,
    AmuletOfAdornment2,
    AmuletOfWisdom,
    AmuletOfCharisma,
    AmuletOfSearching,
    AmuletOfTeleportation,
    AmuletOfSlowDigestion,
    AmuletOfResistAcid,
    AmuletOfTheMagi,
    AmuletOfDoom,
    SilverNecklace,
    GoldNecklace,
    MithrilNecklace,
}

impl AmuletTemplate {
    pub fn vec() -> Vec<Box<dyn template::Template>> {
        vec![
            Box::new(AmuletTemplate::AmuletOfAdornment1),
            Box::new(AmuletTemplate::AmuletOfAdornment2),
            Box::new(AmuletTemplate::AmuletOfWisdom),
            Box::new(AmuletTemplate::AmuletOfCharisma),
            Box::new(AmuletTemplate::AmuletOfSearching),
            Box::new(AmuletTemplate::AmuletOfTeleportation),
            Box::new(AmuletTemplate::AmuletOfSlowDigestion),
            Box::new(AmuletTemplate::AmuletOfResistAcid),
            Box::new(AmuletTemplate::AmuletOfTheMagi),
            Box::new(AmuletTemplate::AmuletOfDoom),
            Box::new(AmuletTemplate::SilverNecklace),
            Box::new(AmuletTemplate::GoldNecklace),
            Box::new(AmuletTemplate::MithrilNecklace),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn template::Template>> {
        AmuletTemplate::vec().into_iter()
    }
}

impl template::Template for AmuletTemplate {
    fn name(&self) -> &str {
        match self {
            AmuletTemplate::AmuletOfAdornment1 => "& Amulet| of Adornment^",
            AmuletTemplate::AmuletOfAdornment2 => "& Amulet| of Adornment^",
            AmuletTemplate::AmuletOfWisdom => "& Amulet| of Wisdom^ (%P1)",
            AmuletTemplate::AmuletOfCharisma => "& Amulet| of Charisma^ (%P1)",
            AmuletTemplate::AmuletOfSearching => "& Amulet| of Searching^ (%P1)",
            AmuletTemplate::AmuletOfTeleportation => "& Amulet| of Teleportation^",
            AmuletTemplate::AmuletOfSlowDigestion => "& Amulet| of Slow Digestion^",
            AmuletTemplate::AmuletOfResistAcid => "& Amulet| of Resist Acid^",
            AmuletTemplate::AmuletOfTheMagi => "& Amulet| of the Magi^",
            AmuletTemplate::AmuletOfDoom => "& Amulet| of Doom^",
            AmuletTemplate::SilverNecklace => "& Silver Necklace~^",
            AmuletTemplate::GoldNecklace => "& Gold Necklace~^",
            AmuletTemplate::MithrilNecklace => "& Mithril Necklace~^",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Amulet }
    fn flags1(&self) -> u64 { 0 }

    fn flags2(&self) -> u64 {
        match self {
            AmuletTemplate::AmuletOfAdornment1 => 0,
            AmuletTemplate::AmuletOfAdornment2 => 0,
            AmuletTemplate::AmuletOfWisdom => 0x00000010,
            AmuletTemplate::AmuletOfCharisma => 0x00000020,
            AmuletTemplate::AmuletOfSearching => 0x00000040,
            AmuletTemplate::AmuletOfTeleportation => 0x80000400,
            AmuletTemplate::AmuletOfSlowDigestion => 0x00000080,
            AmuletTemplate::AmuletOfResistAcid => 0x00100000,
            AmuletTemplate::AmuletOfTheMagi => 0x01800040,
            AmuletTemplate::AmuletOfDoom => 0x8000007F,
            AmuletTemplate::SilverNecklace => 0,
            AmuletTemplate::GoldNecklace => 0,
            AmuletTemplate::MithrilNecklace => 0,
        }
    }

    fn p1(&self) -> i64 {
        match self {
            AmuletTemplate::AmuletOfAdornment1 => 0,
            AmuletTemplate::AmuletOfAdornment2 => 0,
            AmuletTemplate::AmuletOfWisdom => 0,
            AmuletTemplate::AmuletOfCharisma => 0,
            AmuletTemplate::AmuletOfSearching => 0,
            AmuletTemplate::AmuletOfTeleportation => 0,
            AmuletTemplate::AmuletOfSlowDigestion => 0,
            AmuletTemplate::AmuletOfResistAcid => 0,
            AmuletTemplate::AmuletOfTheMagi => 0,
            AmuletTemplate::AmuletOfDoom => -5,
            AmuletTemplate::SilverNecklace => 0,
            AmuletTemplate::GoldNecklace => 0,
            AmuletTemplate::MithrilNecklace => 0,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            AmuletTemplate::AmuletOfAdornment1 => 20,
            AmuletTemplate::AmuletOfAdornment2 => 30,
            AmuletTemplate::AmuletOfWisdom => 300,
            AmuletTemplate::AmuletOfCharisma => 250,
            AmuletTemplate::AmuletOfSearching => 250,
            AmuletTemplate::AmuletOfTeleportation => 0,
            AmuletTemplate::AmuletOfSlowDigestion => 200,
            AmuletTemplate::AmuletOfResistAcid => 300,
            AmuletTemplate::AmuletOfTheMagi => 5000,
            AmuletTemplate::AmuletOfDoom => 0,
            AmuletTemplate::SilverNecklace => 50,
            AmuletTemplate::GoldNecklace => 100,
            AmuletTemplate::MithrilNecklace => 400,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            AmuletTemplate::AmuletOfAdornment1 => 11,
            AmuletTemplate::AmuletOfAdornment2 => 12,
            AmuletTemplate::AmuletOfWisdom => 5,
            AmuletTemplate::AmuletOfCharisma => 6,
            AmuletTemplate::AmuletOfSearching => 7,
            AmuletTemplate::AmuletOfTeleportation => 8,
            AmuletTemplate::AmuletOfSlowDigestion => 9,
            AmuletTemplate::AmuletOfResistAcid => 10,
            AmuletTemplate::AmuletOfTheMagi => 13,
            AmuletTemplate::AmuletOfDoom => 14,
            AmuletTemplate::SilverNecklace => 30,
            AmuletTemplate::GoldNecklace => 40,
            AmuletTemplate::MithrilNecklace => 50,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            AmuletTemplate::AmuletOfAdornment1 => 3,
            AmuletTemplate::AmuletOfAdornment2 => 3,
            AmuletTemplate::AmuletOfWisdom => 3,
            AmuletTemplate::AmuletOfCharisma => 3,
            AmuletTemplate::AmuletOfSearching => 3,
            AmuletTemplate::AmuletOfTeleportation => 3,
            AmuletTemplate::AmuletOfSlowDigestion => 3,
            AmuletTemplate::AmuletOfResistAcid => 3,
            AmuletTemplate::AmuletOfTheMagi => 3,
            AmuletTemplate::AmuletOfDoom => 3,
            AmuletTemplate::SilverNecklace => 5,
            AmuletTemplate::GoldNecklace => 5,
            AmuletTemplate::MithrilNecklace => 5,
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
            AmuletTemplate::AmuletOfAdornment1 => 16,
            AmuletTemplate::AmuletOfAdornment2 => 16,
            AmuletTemplate::AmuletOfWisdom => 20,
            AmuletTemplate::AmuletOfCharisma => 20,
            AmuletTemplate::AmuletOfSearching => 14,
            AmuletTemplate::AmuletOfTeleportation => 14,
            AmuletTemplate::AmuletOfSlowDigestion => 14,
            AmuletTemplate::AmuletOfResistAcid => 24,
            AmuletTemplate::AmuletOfTheMagi => 50,
            AmuletTemplate::AmuletOfDoom => 50,
            AmuletTemplate::SilverNecklace => 0,
            AmuletTemplate::GoldNecklace => 7,
            AmuletTemplate::MithrilNecklace => 9,
        }
    }

    fn is_identified(&self) -> bool { false }
}
