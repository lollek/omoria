use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BracersTemplate {
    BracersOfProtection,
    BracersOfDefense,
    BracersOfShielding,
    MithrilBracers,
    AdamantiteBracers,
    BracersOfWeaponAttraction,
    SilverBraceletOfWarding,
    SilverBracelet,
    GoldBracelet,
    PlatinumBracelet,
    LeatherBracers,
    StuddedLeatherBracers,
    LightPlatedBracers,
    SharkskinBracers,
    DemonhideBracers,
    WyrmhideBracers,
    ChainmailBracers,
    LamellarBracers,
}


impl BracersTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(BracersTemplate::BracersOfProtection),
            Box::new(BracersTemplate::BracersOfDefense),
            Box::new(BracersTemplate::BracersOfShielding),
            Box::new(BracersTemplate::MithrilBracers),
            Box::new(BracersTemplate::AdamantiteBracers),
            Box::new(BracersTemplate::BracersOfWeaponAttraction),
            Box::new(BracersTemplate::SilverBraceletOfWarding),
            Box::new(BracersTemplate::SilverBracelet),
            Box::new(BracersTemplate::GoldBracelet),
            Box::new(BracersTemplate::PlatinumBracelet),
            Box::new(BracersTemplate::LeatherBracers),
            Box::new(BracersTemplate::StuddedLeatherBracers),
            Box::new(BracersTemplate::LightPlatedBracers),
            Box::new(BracersTemplate::SharkskinBracers),
            Box::new(BracersTemplate::DemonhideBracers),
            Box::new(BracersTemplate::WyrmhideBracers),
            Box::new(BracersTemplate::ChainmailBracers),
            Box::new(BracersTemplate::LamellarBracers),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        BracersTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(BracersTemplate::BracersOfProtection),
            2 => Box::new(BracersTemplate::BracersOfDefense),
            3 => Box::new(BracersTemplate::BracersOfShielding),
            4 => Box::new(BracersTemplate::MithrilBracers),
            5 => Box::new(BracersTemplate::AdamantiteBracers),
            6 => Box::new(BracersTemplate::BracersOfWeaponAttraction),
            31 => Box::new(BracersTemplate::SilverBraceletOfWarding),
            30 => Box::new(BracersTemplate::SilverBracelet),
            40 => Box::new(BracersTemplate::GoldBracelet),
            50 => Box::new(BracersTemplate::PlatinumBracelet),
            7 => Box::new(BracersTemplate::LeatherBracers),
            8 => Box::new(BracersTemplate::StuddedLeatherBracers),
            9 => Box::new(BracersTemplate::LightPlatedBracers),
            10 => Box::new(BracersTemplate::SharkskinBracers),
            11 => Box::new(BracersTemplate::DemonhideBracers),
            12 => Box::new(BracersTemplate::WyrmhideBracers),
            13 => Box::new(BracersTemplate::ChainmailBracers),
            14 => Box::new(BracersTemplate::LamellarBracers),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for BracersTemplate {
    fn item_type(&self) -> model::ItemType { model::ItemType::Bracers }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            BracersTemplate::BracersOfProtection => 1200,
            BracersTemplate::BracersOfDefense => 2400,
            BracersTemplate::BracersOfShielding => 3600,
            BracersTemplate::MithrilBracers => 4800,
            BracersTemplate::AdamantiteBracers => 6000,
            BracersTemplate::BracersOfWeaponAttraction => -1200,
            BracersTemplate::SilverBraceletOfWarding => 10000,
            BracersTemplate::SilverBracelet => 25,
            BracersTemplate::GoldBracelet => 50,
            BracersTemplate::PlatinumBracelet => 100,
            BracersTemplate::LeatherBracers => 4,
            BracersTemplate::StuddedLeatherBracers => 25,
            BracersTemplate::LightPlatedBracers => 100,
            BracersTemplate::SharkskinBracers => 200,
            BracersTemplate::DemonhideBracers => 250,
            BracersTemplate::WyrmhideBracers => 500,
            BracersTemplate::ChainmailBracers => 100,
            BracersTemplate::LamellarBracers => 200,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            BracersTemplate::BracersOfProtection => 1,
            BracersTemplate::BracersOfDefense => 2,
            BracersTemplate::BracersOfShielding => 3,
            BracersTemplate::MithrilBracers => 4,
            BracersTemplate::AdamantiteBracers => 5,
            BracersTemplate::BracersOfWeaponAttraction => 6,
            BracersTemplate::SilverBraceletOfWarding => 31,
            BracersTemplate::SilverBracelet => 30,
            BracersTemplate::GoldBracelet => 40,
            BracersTemplate::PlatinumBracelet => 50,
            BracersTemplate::LeatherBracers => 7,
            BracersTemplate::StuddedLeatherBracers => 8,
            BracersTemplate::LightPlatedBracers => 9,
            BracersTemplate::SharkskinBracers => 10,
            BracersTemplate::DemonhideBracers => 11,
            BracersTemplate::WyrmhideBracers => 12,
            BracersTemplate::ChainmailBracers => 13,
            BracersTemplate::LamellarBracers => 14,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            BracersTemplate::BracersOfProtection => 125,
            BracersTemplate::BracersOfDefense => 125,
            BracersTemplate::BracersOfShielding => 125,
            BracersTemplate::MithrilBracers => 125,
            BracersTemplate::AdamantiteBracers => 125,
            BracersTemplate::BracersOfWeaponAttraction => 125,
            BracersTemplate::SilverBraceletOfWarding => 5,
            BracersTemplate::SilverBracelet => 5,
            BracersTemplate::GoldBracelet => 5,
            BracersTemplate::PlatinumBracelet => 5,
            BracersTemplate::LeatherBracers => 5,
            BracersTemplate::StuddedLeatherBracers => 10,
            BracersTemplate::LightPlatedBracers => 30,
            BracersTemplate::SharkskinBracers => 10,
            BracersTemplate::DemonhideBracers => 10,
            BracersTemplate::WyrmhideBracers => 10,
            BracersTemplate::ChainmailBracers => 50,
            BracersTemplate::LamellarBracers => 40,
        }
    }

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }

    fn base_ac(&self) -> i16 {
        match self {
            BracersTemplate::BracersOfProtection => 6,
            BracersTemplate::BracersOfDefense => 7,
            BracersTemplate::BracersOfShielding => 8,
            BracersTemplate::MithrilBracers => 9,
            BracersTemplate::AdamantiteBracers => 10,
            BracersTemplate::BracersOfWeaponAttraction => -6,
            BracersTemplate::SilverBraceletOfWarding => 5,
            BracersTemplate::SilverBracelet => 0,
            BracersTemplate::GoldBracelet => 0,
            BracersTemplate::PlatinumBracelet => 0,
            BracersTemplate::LeatherBracers => 1,
            BracersTemplate::StuddedLeatherBracers => 2,
            BracersTemplate::LightPlatedBracers => 3,
            BracersTemplate::SharkskinBracers => 3,
            BracersTemplate::DemonhideBracers => 4,
            BracersTemplate::WyrmhideBracers => 5,
            BracersTemplate::ChainmailBracers => 5,
            BracersTemplate::LamellarBracers => 7,
        }
    }

    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }

    fn item_level(&self) -> u8 {
        match self {
            BracersTemplate::BracersOfProtection => 35,
            BracersTemplate::BracersOfDefense => 40,
            BracersTemplate::BracersOfShielding => 45,
            BracersTemplate::MithrilBracers => 50,
            BracersTemplate::AdamantiteBracers => 55,
            BracersTemplate::BracersOfWeaponAttraction => 30,
            BracersTemplate::SilverBraceletOfWarding => 50,
            BracersTemplate::SilverBracelet => 2,
            BracersTemplate::GoldBracelet => 5,
            BracersTemplate::PlatinumBracelet => 8,
            BracersTemplate::LeatherBracers => 1,
            BracersTemplate::StuddedLeatherBracers => 5,
            BracersTemplate::LightPlatedBracers => 15,
            BracersTemplate::SharkskinBracers => 30,
            BracersTemplate::DemonhideBracers => 40,
            BracersTemplate::WyrmhideBracers => 50,
            BracersTemplate::ChainmailBracers => 25,
            BracersTemplate::LamellarBracers => 44,
        }
    }

    fn is_identified(&self) -> bool { false }
}

