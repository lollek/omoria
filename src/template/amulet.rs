use misc;
use model;

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
    pub fn iter() -> impl Iterator<Item=AmuletTemplate> {
        [
            AmuletTemplate::AmuletOfAdornment1,
            AmuletTemplate::AmuletOfAdornment2,
            AmuletTemplate::AmuletOfWisdom,
            AmuletTemplate::AmuletOfCharisma,
            AmuletTemplate::AmuletOfSearching,
            AmuletTemplate::AmuletOfTeleportation,
            AmuletTemplate::AmuletOfSlowDigestion,
            AmuletTemplate::AmuletOfResistAcid,
            AmuletTemplate::AmuletOfTheMagi,
            AmuletTemplate::AmuletOfDoom,
            AmuletTemplate::SilverNecklace,
            AmuletTemplate::GoldNecklace,
            AmuletTemplate::MithrilNecklace,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::Amulet as u8,
            flags: 0,
            flags2: self.flags2(),
            p1: self.p1(),
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subval(),
            weight: self.weight(),
            number: 1,
            tohit: 0,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage("0d0"),
            level: 0,
            identified: 0,
        }
    }

    fn name(&self) -> &str {
        match self {
            AmuletTemplate::AmuletOfAdornment1 => "& %A Amulet| of Adornment^",
            AmuletTemplate::AmuletOfAdornment2 => "& %A Amulet| of Adornment^",
            AmuletTemplate::AmuletOfWisdom => "& %A Amulet| of Wisdom^ (%P1)",
            AmuletTemplate::AmuletOfCharisma => "& %A Amulet| of Charisma^ (%P1)",
            AmuletTemplate::AmuletOfSearching => "& %A Amulet| of Searching^ (%P1)",
            AmuletTemplate::AmuletOfTeleportation => "& %A Amulet| of Teleportation^",
            AmuletTemplate::AmuletOfSlowDigestion => "& %A Amulet| of Slow Digestion^",
            AmuletTemplate::AmuletOfResistAcid => "& %A Amulet| of Resist Acid^",
            AmuletTemplate::AmuletOfTheMagi => "& %A Amulet| of the Magi^",
            AmuletTemplate::AmuletOfDoom => "& %A Amulet| of Doom^",
            AmuletTemplate::SilverNecklace => "& Silver Necklace~^",
            AmuletTemplate::GoldNecklace => "& Gold Necklace~^",
            AmuletTemplate::MithrilNecklace => "& Mithril Necklace~^",
        }
    }

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

    fn subval(&self) -> i64 {
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

    fn level(&self) -> u8 {
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
}
