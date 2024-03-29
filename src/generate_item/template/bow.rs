use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{ItemSubType, RangedWeaponSubType},
};

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
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
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

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        BowTemplate::vec().into_iter()
    }
}

impl ItemTemplate for BowTemplate {
    fn name(&self) -> &str {
        match self {
            BowTemplate::Shortbow => "Shortbow (%P0)^ (%P2,%P3)",
            BowTemplate::HuntersBow => "Hunters Bow (%P0)^ (%P2,%P3)",
            BowTemplate::CompositeBow => "Composite Bow (%P0)^ (%P2,%P3)",
            BowTemplate::WarBow => "War Bow (%P0)^ (%P2,%P3)",
            BowTemplate::DoubleBow => "Double Bow (%P0)^ (%P2,%P3)",
            BowTemplate::SiegeBow => "Siege Bow (%P0)^ (%P2,%P3)",
            BowTemplate::WardedBow => "Warded Bow (%P0)^ (%P2,%P3)",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::RangedWeapon
    }
    fn flags1(&self) -> u64 {
        0
    }
    fn flags2(&self) -> u64 {
        0
    }

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

    fn subtype(&self) -> ItemSubType {
        match self {
            BowTemplate::Shortbow => ItemSubType::RangedWeapon(RangedWeaponSubType::Shortbow),
            BowTemplate::HuntersBow => ItemSubType::RangedWeapon(RangedWeaponSubType::HuntersBow),
            BowTemplate::CompositeBow => {
                ItemSubType::RangedWeapon(RangedWeaponSubType::CompositeBow)
            }
            BowTemplate::WarBow => ItemSubType::RangedWeapon(RangedWeaponSubType::WarBow),
            BowTemplate::DoubleBow => ItemSubType::RangedWeapon(RangedWeaponSubType::DoubleBow),
            BowTemplate::SiegeBow => ItemSubType::RangedWeapon(RangedWeaponSubType::SiegeBow),
            BowTemplate::WardedBow => ItemSubType::RangedWeapon(RangedWeaponSubType::WardedBow),
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
        "0d0"
    }

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

    fn is_identified(&self) -> bool {
        false
    }
}
