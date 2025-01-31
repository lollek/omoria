use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{ArrowSubType, BoltSubType, ItemSubType, SlingAmmoSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AmmunitionTemplate {
    Arrow,
    Bolt,
    RoundedPebble,
    IronShot,
}

impl AmmunitionTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(AmmunitionTemplate::Arrow),
            Box::new(AmmunitionTemplate::Bolt),
            Box::new(AmmunitionTemplate::RoundedPebble),
            Box::new(AmmunitionTemplate::IronShot),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        AmmunitionTemplate::vec().into_iter()
    }
}

impl ItemTemplate for AmmunitionTemplate {
    fn name(&self) -> &str {
        match self {
            AmmunitionTemplate::Arrow => "& Arrow~^ (%P2,%P3)",
            AmmunitionTemplate::Bolt => "& Bolt~^ (%P2,%P3)",
            AmmunitionTemplate::RoundedPebble => "& Rounded Pebble~^ (%P2,%P3)",
            AmmunitionTemplate::IronShot => "& Iron Shot~^ (%P2,%P3)",
        }
    }

    fn item_type(&self) -> model::ItemType {
        match self {
            AmmunitionTemplate::Arrow => model::ItemType::Arrow,
            AmmunitionTemplate::Bolt => model::ItemType::Bolt,
            AmmunitionTemplate::RoundedPebble => model::ItemType::SlingAmmo,
            AmmunitionTemplate::IronShot => model::ItemType::SlingAmmo,
        }
    }

    fn flags1(&self) -> u64 {
        0
    }
    fn flags2(&self) -> u64 {
        match self {
            AmmunitionTemplate::Arrow => 0x10000000,
            AmmunitionTemplate::Bolt => 0x10000000,
            AmmunitionTemplate::RoundedPebble => 0,
            AmmunitionTemplate::IronShot => 0,
        }
    }
    fn p1(&self) -> i64 {
        0
    }

    fn cost(&self) -> i64 {
        match self {
            AmmunitionTemplate::Arrow => 1,
            AmmunitionTemplate::Bolt => 1,
            AmmunitionTemplate::RoundedPebble => 1,
            AmmunitionTemplate::IronShot => 2,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            AmmunitionTemplate::Arrow => ItemSubType::Arrow(ArrowSubType::Arrow),
            AmmunitionTemplate::Bolt => ItemSubType::Bolt(BoltSubType::Bolt),
            AmmunitionTemplate::RoundedPebble => {
                ItemSubType::SlingAmmo(SlingAmmoSubType::RoundedPebble)
            }
            AmmunitionTemplate::IronShot => ItemSubType::SlingAmmo(SlingAmmoSubType::IronShot),
        }
    }

    fn weight(&self) -> u16 {
        match self {
            AmmunitionTemplate::Arrow => 2,
            AmmunitionTemplate::Bolt => 3,
            AmmunitionTemplate::RoundedPebble => 4,
            AmmunitionTemplate::IronShot => 5,
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
            AmmunitionTemplate::Arrow => "3d4",
            AmmunitionTemplate::Bolt => "3d5",
            AmmunitionTemplate::RoundedPebble => "3d2",
            AmmunitionTemplate::IronShot => "3d3",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            AmmunitionTemplate::Arrow => 2,
            AmmunitionTemplate::Bolt => 3,
            AmmunitionTemplate::RoundedPebble => 0,
            AmmunitionTemplate::IronShot => 3,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
