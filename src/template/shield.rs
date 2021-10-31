use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ShieldTemplate {
    SmallLeatherShield,
    MediumLeatherShield,
    LargeLeatherShield,
    Buckler,
    KiteShield,
    TowerShield,
    SharkskinShield,
    DemonhideShield,
    WyrmhideShield,
}

impl ShieldTemplate {
    pub fn iter() -> impl Iterator<Item=ShieldTemplate> {
        [
            ShieldTemplate::SmallLeatherShield,
            ShieldTemplate::MediumLeatherShield,
            ShieldTemplate::LargeLeatherShield,
            ShieldTemplate::Buckler,
            ShieldTemplate::KiteShield,
            ShieldTemplate::TowerShield,
            ShieldTemplate::SharkskinShield,
            ShieldTemplate::DemonhideShield,
            ShieldTemplate::WyrmhideShield,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::Shield as u8,
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
            ShieldTemplate::SmallLeatherShield => "Small Leather Shield^ [%P6,%P4]",
            ShieldTemplate::MediumLeatherShield => "Medium Leather Shield^ [%P6,%P4]",
            ShieldTemplate::LargeLeatherShield => "Large Leather Shield^ [%P6,%P4]",
            ShieldTemplate::Buckler => "Buckler^ [%P6,%P4]",
            ShieldTemplate::KiteShield => "Kite Shield^ [%P6,%P4]",
            ShieldTemplate::TowerShield => "Tower Shield^ [%P6,%P4]",
            ShieldTemplate::SharkskinShield => "Sharkskin Shield^ [%P6,%P4]",
            ShieldTemplate::DemonhideShield => "Demonhide Shield^ [%P6,%P4]",
            ShieldTemplate::WyrmhideShield => "Wyrmhide Shield^ [%P6,%P4]",
        }
    }

    fn cost(&self) -> i64 {
        match self {
            ShieldTemplate::SmallLeatherShield => 10,
            ShieldTemplate::MediumLeatherShield => 60,
            ShieldTemplate::LargeLeatherShield => 120,
            ShieldTemplate::Buckler => 50,
            ShieldTemplate::KiteShield => 125,
            ShieldTemplate::TowerShield => 400,
            ShieldTemplate::SharkskinShield => 300,
            ShieldTemplate::DemonhideShield => 700,
            ShieldTemplate::WyrmhideShield => 1000,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            ShieldTemplate::SmallLeatherShield => 1,
            ShieldTemplate::MediumLeatherShield => 2,
            ShieldTemplate::LargeLeatherShield => 3,
            ShieldTemplate::Buckler => 4,
            ShieldTemplate::KiteShield => 5,
            ShieldTemplate::TowerShield => 6,
            ShieldTemplate::SharkskinShield => 7,
            ShieldTemplate::DemonhideShield => 8,
            ShieldTemplate::WyrmhideShield => 9,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            ShieldTemplate::SmallLeatherShield => 50,
            ShieldTemplate::MediumLeatherShield => 75,
            ShieldTemplate::LargeLeatherShield => 100,
            ShieldTemplate::Buckler => 65,
            ShieldTemplate::KiteShield => 90,
            ShieldTemplate::TowerShield => 150,
            ShieldTemplate::SharkskinShield => 75,
            ShieldTemplate::DemonhideShield => 75,
            ShieldTemplate::WyrmhideShield => 75,
        }
    }

    fn ac(&self) -> i16 {
        match self {
            ShieldTemplate::SmallLeatherShield => 2,
            ShieldTemplate::MediumLeatherShield => 3,
            ShieldTemplate::LargeLeatherShield => 4,
            ShieldTemplate::Buckler => 3,
            ShieldTemplate::KiteShield => 10,
            ShieldTemplate::TowerShield => 15,
            ShieldTemplate::SharkskinShield => 10,
            ShieldTemplate::DemonhideShield => 15,
            ShieldTemplate::WyrmhideShield => 20,
        }
    }
    pub fn level(&self) -> u8 {
        match self {
            ShieldTemplate::SmallLeatherShield => 3,
            ShieldTemplate::MediumLeatherShield => 8,
            ShieldTemplate::LargeLeatherShield => 15,
            ShieldTemplate::Buckler => 10,
            ShieldTemplate::KiteShield => 20,
            ShieldTemplate::TowerShield => 30,
            ShieldTemplate::SharkskinShield => 30,
            ShieldTemplate::DemonhideShield => 40,
            ShieldTemplate::WyrmhideShield => 50,
        }
    }

}
