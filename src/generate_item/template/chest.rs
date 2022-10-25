use super::super::item_template::ItemTemplate;
use crate::model;

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
        0
    }

    fn flags2(&self) -> u64 {
        match self {
            ChestTemplate::SmallWoodenChest => 0x0F000000,
            ChestTemplate::LargeWoodenChest => 0x15000000,
            ChestTemplate::SmallIronChest => 0x0F000000,
            ChestTemplate::LargeIronChest => 0x1F000000,
            ChestTemplate::SmallSteelChest => 0x0F000000,
            ChestTemplate::LargeSteelChest => 0x23000000,
        }
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

    fn subtype(&self) -> i64 {
        match self {
            ChestTemplate::SmallWoodenChest => 1,
            ChestTemplate::LargeWoodenChest => 4,
            ChestTemplate::SmallIronChest => 7,
            ChestTemplate::LargeIronChest => 10,
            ChestTemplate::SmallSteelChest => 13,
            ChestTemplate::LargeSteelChest => 16,
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
