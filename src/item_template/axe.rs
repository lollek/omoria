use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AxeTemplate {
    Balestarius,
    BattleAxe,
    BroadAxe,
    HandAxe,
    WarAxe,
    LargeAxe,
    BeardedAxe,
    SilverEdgedAxe,
    ChampionAxe,
}

impl AxeTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(AxeTemplate::Balestarius),
            Box::new(AxeTemplate::BattleAxe),
            Box::new(AxeTemplate::BroadAxe),
            Box::new(AxeTemplate::HandAxe),
            Box::new(AxeTemplate::WarAxe),
            Box::new(AxeTemplate::LargeAxe),
            Box::new(AxeTemplate::BeardedAxe),
            Box::new(AxeTemplate::SilverEdgedAxe),
            Box::new(AxeTemplate::ChampionAxe),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        AxeTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(AxeTemplate::Balestarius),
            3 => Box::new(AxeTemplate::BattleAxe),
            4 => Box::new(AxeTemplate::BroadAxe),
            5 => Box::new(AxeTemplate::HandAxe),
            6 => Box::new(AxeTemplate::WarAxe),
            7 => Box::new(AxeTemplate::LargeAxe),
            8 => Box::new(AxeTemplate::BeardedAxe),
            9 => Box::new(AxeTemplate::SilverEdgedAxe),
            10 => Box::new(AxeTemplate::ChampionAxe),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for AxeTemplate {
    fn item_type(&self) -> model::ItemType { model::ItemType::Axe }
    fn flags1(&self) -> u64 { 0x10000000 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            AxeTemplate::Balestarius => 500,
            AxeTemplate::BattleAxe => 334,
            AxeTemplate::BroadAxe => 304,
            AxeTemplate::HandAxe => 20,
            AxeTemplate::WarAxe => 60,
            AxeTemplate::LargeAxe => 60,
            AxeTemplate::BeardedAxe => 205,
            AxeTemplate::SilverEdgedAxe => 750,
            AxeTemplate::ChampionAxe => 850,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            AxeTemplate::Balestarius => 1,
            AxeTemplate::BattleAxe => 3,
            AxeTemplate::BroadAxe => 4,
            AxeTemplate::HandAxe => 5,
            AxeTemplate::WarAxe => 6,
            AxeTemplate::LargeAxe => 7,
            AxeTemplate::BeardedAxe => 8,
            AxeTemplate::SilverEdgedAxe => 9,
            AxeTemplate::ChampionAxe => 10,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            AxeTemplate::Balestarius => 180,
            AxeTemplate::BattleAxe => 170,
            AxeTemplate::BroadAxe => 160,
            AxeTemplate::HandAxe => 30,
            AxeTemplate::WarAxe => 80,
            AxeTemplate::LargeAxe => 140,
            AxeTemplate::BeardedAxe => 170,
            AxeTemplate::SilverEdgedAxe => 170,
            AxeTemplate::ChampionAxe => 250,
        }
    }

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }

    fn damage(&self) -> &str {
        match self {
            AxeTemplate::Balestarius => "2d8",
            AxeTemplate::BattleAxe => "3d4",
            AxeTemplate::BroadAxe => "2d6",
            AxeTemplate::HandAxe => "1d4",
            AxeTemplate::WarAxe => "1d6",
            AxeTemplate::LargeAxe => "1d9",
            AxeTemplate::BeardedAxe => "2d5",
            AxeTemplate::SilverEdgedAxe => "3d6",
            AxeTemplate::ChampionAxe => "5d3",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            AxeTemplate::Balestarius => 30,
            AxeTemplate::BattleAxe => 13,
            AxeTemplate::BroadAxe => 17,
            AxeTemplate::HandAxe => 1,
            AxeTemplate::WarAxe => 4,
            AxeTemplate::LargeAxe => 7,
            AxeTemplate::BeardedAxe => 9,
            AxeTemplate::SilverEdgedAxe => 30,
            AxeTemplate::ChampionAxe => 40,
        }
    }

    fn is_identified(&self) -> bool { false }
}

