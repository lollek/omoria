use std::borrow::Cow;

use model;
use item_template;
use logic::item_name;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AmmunitionTemplate {
    Arrow,
    Bolt,
    RoundedPebble,
    IronShot,
}

impl AmmunitionTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(AmmunitionTemplate::Arrow),
            Box::new(AmmunitionTemplate::Bolt),
            Box::new(AmmunitionTemplate::RoundedPebble),
            Box::new(AmmunitionTemplate::IronShot),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        AmmunitionTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(AmmunitionTemplate::Arrow),
            2 => Box::new(AmmunitionTemplate::Bolt),
            3 => Box::new(AmmunitionTemplate::RoundedPebble),
            4 => Box::new(AmmunitionTemplate::IronShot),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for AmmunitionTemplate {
    fn name(&self, item: &model::Item) -> String {
        let plural_s = || if item.number == 1 { "" } else { "s" };

        let mut parts = Vec::new();
        parts.push(item_name::number_of(item));
        parts.push(Cow::from(
            match self {
                AmmunitionTemplate::Arrow => format!("Arrow{}", plural_s()),
                AmmunitionTemplate::Bolt => format!("Bolt{}", plural_s()),
                AmmunitionTemplate::RoundedPebble => format!("Rounded Pebble{}", plural_s()),
                AmmunitionTemplate::IronShot => format!("Iron Shot{}", plural_s()),
            }));
        parts.push(item_name::attack_enchantment(&item));
        parts.join("")
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
        match self {
            AmmunitionTemplate::Arrow => 0x10000000,
            AmmunitionTemplate::Bolt => 0x10000000,
            AmmunitionTemplate::RoundedPebble => 0,
            AmmunitionTemplate::IronShot => 0,
        }
    }

    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            AmmunitionTemplate::Arrow => 1,
            AmmunitionTemplate::Bolt => 1,
            AmmunitionTemplate::RoundedPebble => 1,
            AmmunitionTemplate::IronShot => 2,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            AmmunitionTemplate::Arrow => 1,
            AmmunitionTemplate::Bolt => 2,
            AmmunitionTemplate::RoundedPebble => 3,
            AmmunitionTemplate::IronShot => 4,
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

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }

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

    fn is_identified(&self) -> bool { false }
}
