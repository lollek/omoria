use model;

use misc;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum AmmunitionTemplate {
    Arrow,
    Bolt,
    RoundedPebble,
    IronShot,
}

impl AmmunitionTemplate {
    pub fn iter() -> impl Iterator<Item=AmmunitionTemplate> {
        [
            AmmunitionTemplate::Arrow,
            AmmunitionTemplate::Bolt,
            AmmunitionTemplate::RoundedPebble,
            AmmunitionTemplate::IronShot,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: self.r#type() as u8,
            flags: self.flags1(),
            flags2: 0,
            p1: 0,
            cost: self.cost() * model::Currency::Gold.value(),
            subval: 1,
            weight: self.weight(),
            number: 1,
            tohit: 0,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage(self.damage()),
            level: 0,
            identified: 0,
        }
    }

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

    pub fn level(&self) -> u8 {
        match self {
            AmmunitionTemplate::Arrow => 2,
            AmmunitionTemplate::Bolt => 3,
            AmmunitionTemplate::RoundedPebble => 0,
            AmmunitionTemplate::IronShot => 3,
        }
    }
}
