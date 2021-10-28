use model;

use misc;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AmmunitionTemplate {
    Arrow,
    Bolt,
    RoundedPebble,
    IronShot,
}

pub fn generate_ammunition(item_level: u8, template: AmmunitionTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: template.r#type() as u8,
        flags: template.flags1(),
        flags2: 0,
        p1: 0,
        cost: template.cost(),
        subval: 1,
        weight: template.weight(),
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage(template.damage()),
        level: item_level as i8,
        identified: 0,
    }
}

impl AmmunitionTemplate {
    fn name(&self) -> &str {
        match self {
            AmmunitionTemplate::Arrow => "& Arrow~^ (%P2,%P3)",
            AmmunitionTemplate::Bolt => "& Bolt~^ (%P2,%P3)",
            AmmunitionTemplate::RoundedPebble => "& Rounded Pebble~^ (%P2,%P3)",
            AmmunitionTemplate::IronShot => "& Iron Shot~^ (%P2,%P3)",
        }
    }

    fn r#type(&self) -> model::ItemType {
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

    fn cost(&self) -> i64 {
        match self {
            AmmunitionTemplate::Arrow => 1,
            AmmunitionTemplate::Bolt => 1,
            AmmunitionTemplate::RoundedPebble => 1,
            AmmunitionTemplate::IronShot => 2,
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

    fn damage(&self) -> &str {
        match self {
            AmmunitionTemplate::Arrow => "3d4",
            AmmunitionTemplate::Bolt => "3d5",
            AmmunitionTemplate::RoundedPebble => "3d2",
            AmmunitionTemplate::IronShot => "3d3",
        }
    }
}
