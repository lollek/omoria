use super::super::item_template::ItemTemplate;
use crate::generate_item::item_template::default_create;
use crate::generate_item::ItemQuality;
use crate::model::{
    self,
    item_subtype::{AmuletSubType, ItemSubType},
    Item, WornFlag1,
};
use crate::random::randint;

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
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
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

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        AmuletTemplate::vec().into_iter()
    }
}

impl ItemTemplate for AmuletTemplate {
    fn create(&self, item_quality: ItemQuality, _item_level: u8) -> Item {
        let mut item = default_create(self, item_quality);
        match self {
            AmuletTemplate::AmuletOfWisdom | AmuletTemplate::AmuletOfCharisma => {
                if item_quality == ItemQuality::Cursed {
                    item.set_cursed(true);
                    item.p1 = randint(3);
                    item.cost *= -1;
                } else {
                    item.p1 = randint(2);
                    item.cost += item.p1 * 10_000;
                }
            }
            AmuletTemplate::AmuletOfSearching => {
                if item_quality == ItemQuality::Cursed {
                    item.set_cursed(true);
                    item.p1 *= -1;
                    item.cost *= -1;
                } else {
                    item.cost += item.p1 * 10_000;
                }
            }
            _ => {}
        }
        item
    }

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

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Amulet
    }
    fn flags1(&self) -> u64 {
        match self {
            AmuletTemplate::AmuletOfAdornment1 => 0,
            AmuletTemplate::AmuletOfAdornment2 => 0,
            AmuletTemplate::AmuletOfWisdom => WornFlag1::GivesWisdom as u64,
            AmuletTemplate::AmuletOfCharisma => WornFlag1::GivesCharisma as u64,
            AmuletTemplate::AmuletOfSearching => WornFlag1::Searching as u64,
            AmuletTemplate::AmuletOfTeleportation => WornFlag1::RandomTeleportation as u64,
            AmuletTemplate::AmuletOfSlowDigestion => WornFlag1::SlowDigestion as u64,
            AmuletTemplate::AmuletOfResistAcid => WornFlag1::ResistAcid as u64,
            AmuletTemplate::AmuletOfTheMagi => {
                WornFlag1::SeeInvisible as u64
                    | WornFlag1::ResistParalysis as u64
                    | WornFlag1::Searching as u64
            }
            AmuletTemplate::AmuletOfDoom => {
                WornFlag1::Cursed as u64
                    | WornFlag1::Searching as u64
                    | WornFlag1::GivesCharisma as u64
                    | WornFlag1::GivesWisdom as u64
                    | WornFlag1::GivesIntelligence as u64
                    | WornFlag1::GivesConstitution as u64
                    | WornFlag1::GivesDexterity as u64
                    | WornFlag1::GivesStrength as u64
            }
            AmuletTemplate::SilverNecklace => 0,
            AmuletTemplate::GoldNecklace => 0,
            AmuletTemplate::MithrilNecklace => 0,
        }
    }

    fn flags2(&self) -> u64 {
        0
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

    fn subtype(&self) -> ItemSubType {
        match self {
            AmuletTemplate::AmuletOfAdornment1 => {
                ItemSubType::Amulet(AmuletSubType::AmuletOfAdornment1)
            }
            AmuletTemplate::AmuletOfAdornment2 => {
                ItemSubType::Amulet(AmuletSubType::AmuletOfAdornment2)
            }
            AmuletTemplate::AmuletOfWisdom => ItemSubType::Amulet(AmuletSubType::AmuletOfWisdom),
            AmuletTemplate::AmuletOfCharisma => {
                ItemSubType::Amulet(AmuletSubType::AmuletOfCharisma)
            }
            AmuletTemplate::AmuletOfSearching => {
                ItemSubType::Amulet(AmuletSubType::AmuletOfSearching)
            }
            AmuletTemplate::AmuletOfTeleportation => {
                ItemSubType::Amulet(AmuletSubType::AmuletOfTeleportation)
            }
            AmuletTemplate::AmuletOfSlowDigestion => {
                ItemSubType::Amulet(AmuletSubType::AmuletOfSlowDigestion)
            }
            AmuletTemplate::AmuletOfResistAcid => {
                ItemSubType::Amulet(AmuletSubType::AmuletOfResistAcid)
            }
            AmuletTemplate::AmuletOfTheMagi => ItemSubType::Amulet(AmuletSubType::AmuletOfTheMagi),
            AmuletTemplate::AmuletOfDoom => ItemSubType::Amulet(AmuletSubType::AmuletOfDoom),
            AmuletTemplate::SilverNecklace => ItemSubType::Amulet(AmuletSubType::SilverNecklace),
            AmuletTemplate::GoldNecklace => ItemSubType::Amulet(AmuletSubType::GoldNecklace),
            AmuletTemplate::MithrilNecklace => ItemSubType::Amulet(AmuletSubType::MithrilNecklace),
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

    fn number(&self) -> u16 {
        1
    }
    fn modifier_to_hit(&self) -> i16 {
        0
    }
    fn modifier_to_damage(&self) -> i16 {
        0
    }
    fn base_ac(&self) -> i16 {
        0
    }
    fn modifier_to_ac(&self) -> i16 {
        0
    }
    fn damage(&self) -> &str {
        "1d1"
    }

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

    fn is_identified(&self) -> bool {
        false
    }
}
