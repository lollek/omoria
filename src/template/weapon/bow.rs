use misc;
use model;

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

pub fn generate_bow(item_level: u8, template: BowTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::RangedWeapon as u8,
        flags: 0,
        flags2: 0,
        p1: template.p1(),
        cost: template.cost(),
        subval: template.subval(),
        weight: template.weight(),
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: item_level as i8,
        identified: 0,
    }
}

impl BowTemplate {
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

    fn subval(&self) -> i64 {
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
}

