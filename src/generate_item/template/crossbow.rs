use crate::generate_item::item_template::create_ranged_weapon;
use crate::generate_item::ItemQuality;
use super::super::item_template::ItemTemplate;
use crate::model::{self, item_subtype::{ItemSubType, RangedWeaponSubType}, Item};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum CrossbowTemplate {
    SiegeCrossbow,
    Ballista,
    LightCrossbow,
    HeavyCrossbow,
}

impl CrossbowTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(CrossbowTemplate::SiegeCrossbow),
            Box::new(CrossbowTemplate::Ballista),
            Box::new(CrossbowTemplate::LightCrossbow),
            Box::new(CrossbowTemplate::HeavyCrossbow),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        CrossbowTemplate::vec().into_iter()
    }
}

impl ItemTemplate for CrossbowTemplate {
    fn create(&self, item_quality: ItemQuality, _item_level: u8) -> Item {
        create_ranged_weapon(self, item_quality)
    }

    fn name(&self) -> &str {
        match self {
            CrossbowTemplate::SiegeCrossbow => "Siege Crossbow (%P0)^ (%P2,%P3)",
            CrossbowTemplate::Ballista => "Ballista (%P0)^ (%P2,%P3)",
            CrossbowTemplate::LightCrossbow => "Light Crossbow (%P0)^ (%P2,%P3)",
            CrossbowTemplate::HeavyCrossbow => "Heavy Crossbow (%P0)^ (%P2,%P3)",
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
            CrossbowTemplate::SiegeCrossbow => 6,
            CrossbowTemplate::Ballista => 8,
            CrossbowTemplate::LightCrossbow => 2,
            CrossbowTemplate::HeavyCrossbow => 4,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            CrossbowTemplate::SiegeCrossbow => 140,
            CrossbowTemplate::Ballista => 300,
            CrossbowTemplate::LightCrossbow => 50,
            CrossbowTemplate::HeavyCrossbow => 120,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            CrossbowTemplate::SiegeCrossbow => {
                ItemSubType::RangedWeapon(RangedWeaponSubType::SiegeCrossbow)
            }
            CrossbowTemplate::Ballista => ItemSubType::RangedWeapon(RangedWeaponSubType::Ballista),
            CrossbowTemplate::LightCrossbow => {
                ItemSubType::RangedWeapon(RangedWeaponSubType::LightCrossbow)
            }
            CrossbowTemplate::HeavyCrossbow => {
                ItemSubType::RangedWeapon(RangedWeaponSubType::HeavyCrossbow)
            }
        }
    }

    fn weight(&self) -> u16 {
        match self {
            CrossbowTemplate::SiegeCrossbow => 200,
            CrossbowTemplate::Ballista => 250,
            CrossbowTemplate::LightCrossbow => 110,
            CrossbowTemplate::HeavyCrossbow => 200,
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
            CrossbowTemplate::SiegeCrossbow => 15,
            CrossbowTemplate::Ballista => 30,
            CrossbowTemplate::LightCrossbow => 3,
            CrossbowTemplate::HeavyCrossbow => 10,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
