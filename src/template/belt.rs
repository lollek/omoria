use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BeltTemplate {
    Sash,
    LightBelt,
    Belt,
    HeavyBelt,
    LightPlatedBelt,
    SharkskinBelt,
    DemonhideBelt,
    WyrmhideBelt,
}


impl BeltTemplate {
    pub fn iter() -> impl Iterator<Item=BeltTemplate> {
        [
            BeltTemplate::Sash,
            BeltTemplate::LightBelt,
            BeltTemplate::Belt,
            BeltTemplate::HeavyBelt,
            BeltTemplate::LightPlatedBelt,
            BeltTemplate::SharkskinBelt,
            BeltTemplate::DemonhideBelt,
            BeltTemplate::WyrmhideBelt,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::Belt as u8,
            flags: 0,
            flags2: 0,
            p1: 0,
            cost: self.cost(),
            subval: self.subval(),
            weight: self.weight(),
            number: 1,
            tohit: 0,
            todam: 0,
            ac: self.ac(),
            toac: 0,
            damage: misc::rs2item_damage("0d0"),
            level: 0,
            identified: 0,
        }
    }

    fn name(&self) -> &str {
        match self {
            BeltTemplate::Sash => "Sash^ [%P6,%P4]",
            BeltTemplate::LightBelt => "Light Belt^ [%P6,%P4]",
            BeltTemplate::Belt => "Belt^ [%P6,%P4]",
            BeltTemplate::HeavyBelt => "Heavy Belt^ [%P6,%P4]",
            BeltTemplate::LightPlatedBelt => "Light Plated Belt^ [%P6,%P4]",
            BeltTemplate::SharkskinBelt => "Sharkskin Belt^ [%P6,%P4]",
            BeltTemplate::DemonhideBelt => "Demonhide Belt^ [%P6,%P4]",
            BeltTemplate::WyrmhideBelt => "Wyrmhide Belt^ [%P6,%P4]",
        }
    }

    fn cost(&self) -> i64 {
        match self {
            BeltTemplate::Sash => 1,
            BeltTemplate::LightBelt => 5,
            BeltTemplate::Belt => 7,
            BeltTemplate::HeavyBelt => 20,
            BeltTemplate::LightPlatedBelt => 50,
            BeltTemplate::SharkskinBelt => 200,
            BeltTemplate::DemonhideBelt => 300,
            BeltTemplate::WyrmhideBelt => 500,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            BeltTemplate::Sash => 1,
            BeltTemplate::LightBelt => 2,
            BeltTemplate::Belt => 3,
            BeltTemplate::HeavyBelt => 4,
            BeltTemplate::LightPlatedBelt => 5,
            BeltTemplate::SharkskinBelt => 6,
            BeltTemplate::DemonhideBelt => 7,
            BeltTemplate::WyrmhideBelt => 8,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            BeltTemplate::Sash => 1,
            BeltTemplate::LightBelt => 5,
            BeltTemplate::Belt => 10,
            BeltTemplate::HeavyBelt => 15,
            BeltTemplate::LightPlatedBelt => 30,
            BeltTemplate::SharkskinBelt => 10,
            BeltTemplate::DemonhideBelt => 10,
            BeltTemplate::WyrmhideBelt => 10,
        }
    }

    fn ac(&self) -> i16 {
        match self {
            BeltTemplate::Sash => 0,
            BeltTemplate::LightBelt => 0,
            BeltTemplate::Belt => 0,
            BeltTemplate::HeavyBelt => 1,
            BeltTemplate::LightPlatedBelt => 2,
            BeltTemplate::SharkskinBelt => 3,
            BeltTemplate::DemonhideBelt => 4,
            BeltTemplate::WyrmhideBelt => 5,
        }
    }

    fn level(&self) -> u8 {
        match self {
            BeltTemplate::Sash => 0,
            BeltTemplate::LightBelt => 0,
            BeltTemplate::Belt => 0,
            BeltTemplate::HeavyBelt => 6,
            BeltTemplate::LightPlatedBelt => 15,
            BeltTemplate::SharkskinBelt => 30,
            BeltTemplate::DemonhideBelt => 40,
            BeltTemplate::WyrmhideBelt => 50,
        }
    }
}
