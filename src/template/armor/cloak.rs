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

pub fn generate_cloak(item_level: u8, template: CloakTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::Bracers as u8,
        flags: 0,
        flags2: 0,
        p1: 0,
        cost: template.cost(),
        subval: template.subval(),
        weight: template.weight(),
        number: 1,
        tohit: 0,
        todam: 0,
        ac: template.ac(),
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: item_level as i8,
        identified: 0,
    }
}

impl CloakTemplate {
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
}
