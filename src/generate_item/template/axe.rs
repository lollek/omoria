use super::super::item_template::ItemTemplate;
use crate::model;

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
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
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

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        AxeTemplate::vec().into_iter()
    }
}

impl ItemTemplate for AxeTemplate {
    fn name(&self) -> &str {
        match self {
            AxeTemplate::Balestarius => "Balestarius (%P0)^ (%P2,%P3)",
            AxeTemplate::BattleAxe => "Battle Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::BroadAxe => "Broad Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::HandAxe => "Hand Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::WarAxe => "War Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::LargeAxe => "Large Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::BeardedAxe => "Bearded Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::SilverEdgedAxe => "Silved Edged Axe (%P0)^ (%P2,%P3)",
            AxeTemplate::ChampionAxe => "Champion Axe (%P0)^ (%P2,%P3)",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::HaftedWeapon
    }
    fn flags1(&self) -> u64 {
        0x10000000
    }
    fn flags2(&self) -> u64 {
        0
    }
    fn p1(&self) -> i64 {
        0
    }

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

    fn is_identified(&self) -> bool {
        false
    }
}
