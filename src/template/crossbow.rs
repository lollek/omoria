use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum CrossbowTemplate {
    SiegeCrossbow,
    Ballista,
    LightCrossbow,
    HeavyCrossbow,
}


impl CrossbowTemplate {
    pub fn iter() -> impl Iterator<Item=CrossbowTemplate> {
        [
            CrossbowTemplate::SiegeCrossbow,
            CrossbowTemplate::Ballista,
            CrossbowTemplate::LightCrossbow,
            CrossbowTemplate::HeavyCrossbow,
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
            CrossbowTemplate::SiegeCrossbow => "Siege Crossbow (%P0)^ (%P2,%P3)",
            CrossbowTemplate::Ballista => "Ballista (%P0)^ (%P2,%P3)",
            CrossbowTemplate::LightCrossbow => "Light Crossbow (%P0)^ (%P2,%P3)",
            CrossbowTemplate::HeavyCrossbow => "Heavy Crossbow (%P0)^ (%P2,%P3)",
       }
    }

    fn p1(&self) -> i64 {
        match self {
            CrossbowTemplate::SiegeCrossbow => 5,
            CrossbowTemplate::Ballista => 6,
            CrossbowTemplate::LightCrossbow => 6,
            CrossbowTemplate::HeavyCrossbow => 7,
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

    fn subval(&self) -> i64 {
        match self {
            CrossbowTemplate::SiegeCrossbow => 10,
            CrossbowTemplate::Ballista => 11,
            CrossbowTemplate::LightCrossbow => 12,
            CrossbowTemplate::HeavyCrossbow => 12,
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

    pub fn level(&self) -> u8 {
        match self {
            CrossbowTemplate::SiegeCrossbow => 15,
            CrossbowTemplate::Ballista => 30,
            CrossbowTemplate::LightCrossbow => 3,
            CrossbowTemplate::HeavyCrossbow => 10,
        }
    }
}
