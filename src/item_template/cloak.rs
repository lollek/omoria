use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum CloakTemplate {
    LightCloak,
    HeavyCloak,
    SharkskinCloak,
    DemonhideCloak,
    WyrmhideCloak,
}

impl CloakTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(CloakTemplate::LightCloak),
            Box::new(CloakTemplate::HeavyCloak),
            Box::new(CloakTemplate::SharkskinCloak),
            Box::new(CloakTemplate::DemonhideCloak),
            Box::new(CloakTemplate::WyrmhideCloak),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        CloakTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(CloakTemplate::LightCloak),
            2 => Box::new(CloakTemplate::HeavyCloak),
            3 => Box::new(CloakTemplate::SharkskinCloak),
            4 => Box::new(CloakTemplate::DemonhideCloak),
            5 => Box::new(CloakTemplate::WyrmhideCloak),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for CloakTemplate {

    fn name(&self) -> &str {
        match self {
            CloakTemplate::LightCloak => "Light Cloak^ [%P6,%P4]",
            CloakTemplate::HeavyCloak => "Heavy Cloak^ [%P6,%P4]",
            CloakTemplate::SharkskinCloak => "Sharkskin Cloak^ [%P6,%P4]",
            CloakTemplate::DemonhideCloak => "Demonhide Cloak^ [%P6,%P4]",
            CloakTemplate::WyrmhideCloak => "Wyrmhide Cloak^ [%P6,%P4]",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Bracers }

    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            CloakTemplate::LightCloak => 3,
            CloakTemplate::HeavyCloak => 10,
            CloakTemplate::SharkskinCloak => 500,
            CloakTemplate::DemonhideCloak => 800,
            CloakTemplate::WyrmhideCloak => 1500,
        }
    }

    fn subtype(&self) -> i64 {
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

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }

    fn base_ac(&self) -> i16 {
        match self {
            CloakTemplate::LightCloak => 1,
            CloakTemplate::HeavyCloak => 2,
            CloakTemplate::SharkskinCloak => 4,
            CloakTemplate::DemonhideCloak => 6,
            CloakTemplate::WyrmhideCloak => 8,
        }
    }

    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "0d0" }

    fn item_level(&self) -> u8 {
        match self {
            CloakTemplate::LightCloak => 0,
            CloakTemplate::HeavyCloak => 0,
            CloakTemplate::SharkskinCloak => 30,
            CloakTemplate::DemonhideCloak => 40,
            CloakTemplate::WyrmhideCloak => 50,
        }
    }

    fn is_identified(&self) -> bool { false }
}
