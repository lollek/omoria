use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum CloakTemplate {
    LightCloak,
    HeavyCloak,
    SharkskinCloak,
    DemonhideCloak,
    WyrmhideCloak,
}

impl CloakTemplate {
    pub fn iter() -> impl Iterator<Item=CloakTemplate> {
        [
            CloakTemplate::LightCloak,
            CloakTemplate::HeavyCloak,
            CloakTemplate::SharkskinCloak,
            CloakTemplate::DemonhideCloak,
            CloakTemplate::WyrmhideCloak,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::Bracers as u8,
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
            CloakTemplate::LightCloak => "Light Cloak^ [%P6,%P4]",
            CloakTemplate::HeavyCloak => "Heavy Cloak^ [%P6,%P4]",
            CloakTemplate::SharkskinCloak => "Sharkskin Cloak^ [%P6,%P4]",
            CloakTemplate::DemonhideCloak => "Demonhide Cloak^ [%P6,%P4]",
            CloakTemplate::WyrmhideCloak => "Wyrmhide Cloak^ [%P6,%P4]",
        }
    }

    fn cost(&self) -> i64 {
        match self {
            CloakTemplate::LightCloak => 3,
            CloakTemplate::HeavyCloak => 10,
            CloakTemplate::SharkskinCloak => 500,
            CloakTemplate::DemonhideCloak => 800,
            CloakTemplate::WyrmhideCloak => 1500,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            CloakTemplate::LightCloak => 1,
            CloakTemplate::HeavyCloak => 2,
            CloakTemplate::SharkskinCloak => 3,
            CloakTemplate::DemonhideCloak => 4,
            CloakTemplate::WyrmhideCloak => 5,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            CloakTemplate::LightCloak => 10,
            CloakTemplate::HeavyCloak => 40,
            CloakTemplate::SharkskinCloak => 100,
            CloakTemplate::DemonhideCloak => 100,
            CloakTemplate::WyrmhideCloak => 100,
        }
    }

    fn ac(&self) -> i16 {
        match self {
            CloakTemplate::LightCloak => 1,
            CloakTemplate::HeavyCloak => 2,
            CloakTemplate::SharkskinCloak => 4,
            CloakTemplate::DemonhideCloak => 6,
            CloakTemplate::WyrmhideCloak => 8,
        }
    }

    fn level(&self) -> u8 {
        match self {
            CloakTemplate::LightCloak => 0,
            CloakTemplate::HeavyCloak => 0,
            CloakTemplate::SharkskinCloak => 30,
            CloakTemplate::DemonhideCloak => 40,
            CloakTemplate::WyrmhideCloak => 50,
        }
    }
}
