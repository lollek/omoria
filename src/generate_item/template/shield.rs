use crate::model;
use super::super::item_template::ItemTemplate;

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
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(ShieldTemplate::SmallLeatherShield),
            Box::new(ShieldTemplate::MediumLeatherShield),
            Box::new(ShieldTemplate::LargeLeatherShield),
            Box::new(ShieldTemplate::Buckler),
            Box::new(ShieldTemplate::KiteShield),
            Box::new(ShieldTemplate::TowerShield),
            Box::new(ShieldTemplate::SharkskinShield),
            Box::new(ShieldTemplate::DemonhideShield),
            Box::new(ShieldTemplate::WyrmhideShield),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn ItemTemplate>> {
        ShieldTemplate::vec().into_iter()
    }
}

impl ItemTemplate for ShieldTemplate {

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

    fn item_type(&self) -> model::ItemType { model::ItemType::Shield }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }

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

    fn subtype(&self) -> i64 {
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

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }

    fn base_ac(&self) -> i16 {
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

    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "0d0" }

    fn item_level(&self) -> u8 {
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

    fn is_identified(&self) -> bool { false }
}
