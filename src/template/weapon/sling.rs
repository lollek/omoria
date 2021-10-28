use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SlingTemplate {
    Sling,
}

pub fn generate_sling(item_level: u8, template: SlingTemplate) -> model::Item {
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

impl SlingTemplate {
    fn name(&self) -> &str {
        match self {
            SlingTemplate::Sling => "Sling (%P0)^ (%P2,%P3)",
       }
    }

    fn p1(&self) -> i64 {
        match self {
            SlingTemplate::Sling => 1,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            SlingTemplate::Sling => 5,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            SlingTemplate::Sling => 20,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            SlingTemplate::Sling => 5,
        }
    }
}
