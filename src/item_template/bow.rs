use std::borrow::Cow;

use model;
use item_template;
use logic::item_name;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BowTemplate {
    Shortbow,
    HuntersBow,
    CompositeBow,
    WarBow,
    DoubleBow,
    SiegeBow,
    WardedBow,
}


impl BowTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(BowTemplate::Shortbow),
            Box::new(BowTemplate::HuntersBow),
            Box::new(BowTemplate::CompositeBow),
            Box::new(BowTemplate::WarBow),
            Box::new(BowTemplate::DoubleBow),
            Box::new(BowTemplate::SiegeBow),
            Box::new(BowTemplate::WardedBow),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        BowTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(BowTemplate::Shortbow),
            2 => Box::new(BowTemplate::HuntersBow),
            3 => Box::new(BowTemplate::CompositeBow),
            4 => Box::new(BowTemplate::WarBow),
            5 => Box::new(BowTemplate::DoubleBow),
            6 => Box::new(BowTemplate::SiegeBow),
            7 => Box::new(BowTemplate::WardedBow),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for BowTemplate {
    fn name(&self, item: &model::Item) -> String {
        item_name::generate_weapon_name(item,
            Cow::from(match self {
                BowTemplate::Shortbow => "Shortbow",
                BowTemplate::HuntersBow => "Hunters Bow",
                BowTemplate::CompositeBow => "Composite Bow",
                BowTemplate::WarBow => "War Bow",
                BowTemplate::DoubleBow => "Double Bow",
                BowTemplate::SiegeBow => "Siege Bow",
                BowTemplate::WardedBow => "Warded Bow",
            }))
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Bow }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }

    fn p1(&self) -> i64 {
        match self {
            BowTemplate::Shortbow => 2,
            BowTemplate::HuntersBow => 3,
            BowTemplate::CompositeBow => 4,
            BowTemplate::WarBow => 5,
            BowTemplate::DoubleBow => 6,
            BowTemplate::SiegeBow => 7,
            BowTemplate::WardedBow => 8,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            BowTemplate::Shortbow => 50,
            BowTemplate::HuntersBow => 120,
            BowTemplate::CompositeBow => 240,
            BowTemplate::WarBow => 150,
            BowTemplate::DoubleBow => 170,
            BowTemplate::SiegeBow => 200,
            BowTemplate::WardedBow => 220,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            BowTemplate::Shortbow => 1,
            BowTemplate::HuntersBow => 2,
            BowTemplate::CompositeBow => 3,
            BowTemplate::WarBow => 4,
            BowTemplate::DoubleBow => 5,
            BowTemplate::SiegeBow => 6,
            BowTemplate::WardedBow => 7,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            BowTemplate::Shortbow => 30,
            BowTemplate::HuntersBow => 40,
            BowTemplate::CompositeBow => 40,
            BowTemplate::WarBow => 40,
            BowTemplate::DoubleBow => 40,
            BowTemplate::SiegeBow => 40,
            BowTemplate::WardedBow => 40,
        }
    }

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }

    fn damage(&self) -> &str { "0d0" }

    fn item_level(&self) -> u8 {
        match self {
            BowTemplate::Shortbow => 3,
            BowTemplate::HuntersBow => 10,
            BowTemplate::CompositeBow => 40,
            BowTemplate::WarBow => 15,
            BowTemplate::DoubleBow => 20,
            BowTemplate::SiegeBow => 25,
            BowTemplate::WardedBow => 30,
        }
    }

    fn is_identified(&self) -> bool { false }
}

