use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum MagicBookTemplate {
    BeginnersMagic,
    Magic1,
    Magic2,
    MagesGuideToPower,
}

impl MagicBookTemplate {
    pub fn iter() -> impl Iterator<Item=MagicBookTemplate> {
        [
            MagicBookTemplate::BeginnersMagic,
            MagicBookTemplate::Magic1,
            MagicBookTemplate::Magic2,
            MagicBookTemplate::MagesGuideToPower,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::MagicBook as u8,
            flags: self.flags1(),
            flags2: self.flags2(),
            p1: 0,
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subval(),
            weight: 60,
            number: 1,
            tohit: -100,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage("1d1"),
            level: 0,
            identified: 0,
        }
    }

    fn name(&self) -> &str {
        match self {
            MagicBookTemplate::BeginnersMagic => "& Book of Magic Spells [Beginners-Magik]",
            MagicBookTemplate::Magic1 => "& Book of Magic Spells [Magik I]",
            MagicBookTemplate::Magic2 => "& Book of Magic Spells [Magik II]",
            MagicBookTemplate::MagesGuideToPower => "& Book of Magic Spells [The Mages Guide to Power]",
        }
    }

    fn flags1(&self) -> u64 {
        match self {
            MagicBookTemplate::BeginnersMagic => 0x00000000,
            MagicBookTemplate::Magic1 => 0x00000000,
            MagicBookTemplate::Magic2 => 0x00000000,
            MagicBookTemplate::MagesGuideToPower => 0x000001FF,
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            MagicBookTemplate::BeginnersMagic => 0x0000007F,
            MagicBookTemplate::Magic1 => 0x0007FF80,
            MagicBookTemplate::Magic2 => 0x7FF80000,
            MagicBookTemplate::MagesGuideToPower => 0x00000000,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            MagicBookTemplate::BeginnersMagic => 25,
            MagicBookTemplate::Magic1 => 100,
            MagicBookTemplate::Magic2 => 400,
            MagicBookTemplate::MagesGuideToPower => 800,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            MagicBookTemplate::BeginnersMagic => 257,
            MagicBookTemplate::Magic1 => 258,
            MagicBookTemplate::Magic2 => 259,
            MagicBookTemplate::MagesGuideToPower => 261,
        }
    }
}
