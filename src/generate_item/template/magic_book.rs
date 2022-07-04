use model;
use super::super::item_template::ItemTemplate;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum MagicBookTemplate {
    BeginnersMagic,
    Magic1,
    Magic2,
    MagesGuideToPower,
}

impl MagicBookTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(MagicBookTemplate::BeginnersMagic),
            Box::new(MagicBookTemplate::Magic1),
            Box::new(MagicBookTemplate::Magic2),
            Box::new(MagicBookTemplate::MagesGuideToPower),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn ItemTemplate>> {
        MagicBookTemplate::vec().into_iter()
    }
}

impl ItemTemplate for MagicBookTemplate {
    fn name(&self) -> &str {
        match self {
            MagicBookTemplate::BeginnersMagic => "& Book of Magic Spells [Beginners-Magik]",
            MagicBookTemplate::Magic1 => "& Book of Magic Spells [Magik I]",
            MagicBookTemplate::Magic2 => "& Book of Magic Spells [Magik II]",
            MagicBookTemplate::MagesGuideToPower => "& Book of Magic Spells [The Mages Guide to Power]",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::MagicBook }

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

    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            MagicBookTemplate::BeginnersMagic => 25,
            MagicBookTemplate::Magic1 => 100,
            MagicBookTemplate::Magic2 => 400,
            MagicBookTemplate::MagesGuideToPower => 800,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            MagicBookTemplate::BeginnersMagic => 257,
            MagicBookTemplate::Magic1 => 258,
            MagicBookTemplate::Magic2 => 259,
            MagicBookTemplate::MagesGuideToPower => 261,
        }
    }

    fn weight(&self) -> u16 { 60 }
    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }
    fn item_level(&self) -> u8 { 0 }
    fn is_identified(&self) -> bool { false }
}
