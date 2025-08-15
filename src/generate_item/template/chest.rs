use super::super::item_template::ItemTemplate;
use crate::generate_item::item_template::default_create;
use crate::generate_item::ItemQuality;
use crate::model::{
    self,
    item_subtype::{ChestSubType, ItemSubType},
    ChestFlags1, Item,
};
use crate::random::randint;
use std::cmp::max;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ChestTemplate {
    SmallWoodenChest,
    LargeWoodenChest,
    SmallIronChest,
    LargeIronChest,
    SmallSteelChest,
    LargeSteelChest,
}

impl ChestTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(ChestTemplate::SmallWoodenChest),
            Box::new(ChestTemplate::LargeWoodenChest),
            Box::new(ChestTemplate::SmallIronChest),
            Box::new(ChestTemplate::LargeIronChest),
            Box::new(ChestTemplate::SmallSteelChest),
            Box::new(ChestTemplate::LargeSteelChest),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        ChestTemplate::vec().into_iter()
    }
}

impl ItemTemplate for ChestTemplate {
    fn create(&self, item_quality: ItemQuality, item_level: u8) -> Item {
        let mut item = default_create(self, item_quality);
        /*
         * Items inside the chest will be created as if
         * found on dungeon level p1.
         */
        item.p1 = max(1, item_level as i64 + randint(10) - 5);
        item.apply_chestflag1(ChestFlags1::Locked);
        match randint(item_level as i64) + 4 {
            1..=2 => {}
            3..=4 => item.apply_chestflag1(ChestFlags1::PoisonNeedle1),
            5..=6 => item.apply_chestflag1(ChestFlags1::PoisonNeedle2),
            7..=9 => item.apply_chestflag1(ChestFlags1::GasTrap),
            10..=11 => item.apply_chestflag1(ChestFlags1::ExplosiveTrap),
            12..=14 => item.apply_chestflag1(ChestFlags1::SummoningRunes),
            15..=17 => {
                item.apply_chestflag1(ChestFlags1::PoisonNeedle1);
                item.apply_chestflag1(ChestFlags1::PoisonNeedle2);
                item.apply_chestflag1(ChestFlags1::GasTrap);
            }
            _ => {
                item.apply_chestflag1(ChestFlags1::ExplosiveTrap);
                item.apply_chestflag1(ChestFlags1::SummoningRunes);
            }
        }
        item
    }
    fn name(&self) -> &str {
        match self {
            ChestTemplate::SmallWoodenChest => "& Small wooden chest",
            ChestTemplate::LargeWoodenChest => "& Large wooden chest",
            ChestTemplate::SmallIronChest => "& Small iron chest",
            ChestTemplate::LargeIronChest => "& Large iron chest",
            ChestTemplate::SmallSteelChest => "& Small steel chest",
            ChestTemplate::LargeSteelChest => "& Large steel chest",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Chest
    }
    fn flags1(&self) -> u64 {
        match self {
            ChestTemplate::SmallWoodenChest => 0x0F000000,
            ChestTemplate::LargeWoodenChest => 0x15000000,
            ChestTemplate::SmallIronChest => 0x0F000000,
            ChestTemplate::LargeIronChest => 0x1F000000,
            ChestTemplate::SmallSteelChest => 0x0F000000,
            ChestTemplate::LargeSteelChest => 0x23000000,
        }
    }

    fn flags2(&self) -> u64 {
        0
    }

    fn p1(&self) -> i64 {
        0
    }

    fn cost(&self) -> i64 {
        match self {
            ChestTemplate::SmallWoodenChest => 300,
            ChestTemplate::LargeWoodenChest => 320,
            ChestTemplate::SmallIronChest => 420,
            ChestTemplate::LargeIronChest => 520,
            ChestTemplate::SmallSteelChest => 520,
            ChestTemplate::LargeSteelChest => 620,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            ChestTemplate::SmallWoodenChest => ItemSubType::Chest(ChestSubType::SmallWoodenChest),
            ChestTemplate::LargeWoodenChest => ItemSubType::Chest(ChestSubType::LargeWoodenChest),
            ChestTemplate::SmallIronChest => ItemSubType::Chest(ChestSubType::SmallIronChest),
            ChestTemplate::LargeIronChest => ItemSubType::Chest(ChestSubType::LargeIronChest),
            ChestTemplate::SmallSteelChest => ItemSubType::Chest(ChestSubType::SmallSteelChest),
            ChestTemplate::LargeSteelChest => ItemSubType::Chest(ChestSubType::LargeSteelChest),
        }
    }

    fn weight(&self) -> u16 {
        match self {
            ChestTemplate::SmallWoodenChest => 250,
            ChestTemplate::LargeWoodenChest => 500,
            ChestTemplate::SmallIronChest => 300,
            ChestTemplate::LargeIronChest => 1000,
            ChestTemplate::SmallSteelChest => 500,
            ChestTemplate::LargeSteelChest => 1000,
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
        match self {
            ChestTemplate::SmallWoodenChest => "2d3",
            ChestTemplate::LargeWoodenChest => "2d5",
            ChestTemplate::SmallIronChest => "2d4",
            ChestTemplate::LargeIronChest => "2d6",
            ChestTemplate::SmallSteelChest => "2d4",
            ChestTemplate::LargeSteelChest => "2d6",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            ChestTemplate::SmallWoodenChest => 7,
            ChestTemplate::LargeWoodenChest => 15,
            ChestTemplate::SmallIronChest => 25,
            ChestTemplate::LargeIronChest => 35,
            ChestTemplate::SmallSteelChest => 45,
            ChestTemplate::LargeSteelChest => 50,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
