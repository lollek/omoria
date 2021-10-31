use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SlingTemplate {
    Sling,
}


impl SlingTemplate {
    pub fn iter() -> impl Iterator<Item=SlingTemplate> {
        [
            SlingTemplate::Sling,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::RangedWeapon as u8,
            flags: 0,
            flags2: 0,
            p1: self.p1(),
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subval(),
            weight: self.weight(),
            number: 1,
            tohit: 0,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage("0d0"),
            level: 0,
            identified: 0,
        }
    }

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

    pub fn level(&self) -> u8 {
        match self {
            SlingTemplate::Sling => 0,
        }
    }
}
