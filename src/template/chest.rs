use misc;
use model;

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
    pub fn iter() -> impl Iterator<Item=ChestTemplate> {
        [
            ChestTemplate::SmallWoodenChest,
            ChestTemplate::LargeWoodenChest,
            ChestTemplate::SmallIronChest,
            ChestTemplate::LargeIronChest,
            ChestTemplate::SmallSteelChest,
            ChestTemplate::LargeSteelChest,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::Chest as u8,
            flags: 0,
            flags2: self.flags2(),
            p1: 0,
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subval(),
            weight: self.weight(),
            number: 1,
            tohit: 0,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage(self.damage()),
            level: 0,
            identified: 0,
        }
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

    fn subval(&self) -> i64 {
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

    fn level(&self) -> u8 {
        match self {
            ChestTemplate::SmallWoodenChest => 7,
            ChestTemplate::LargeWoodenChest => 15,
            ChestTemplate::SmallIronChest => 25,
            ChestTemplate::LargeIronChest => 35,
            ChestTemplate::SmallSteelChest => 45,
            ChestTemplate::LargeSteelChest => 50,
        }
    }
}

