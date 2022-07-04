use model;
use super::super::item_template::ItemTemplate;

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
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
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

    pub fn iter() -> impl Iterator<Item=Box<dyn ItemTemplate>> {
        BracersTemplate::vec().into_iter()
    }
}

impl ItemTemplate for BracersTemplate {
    fn name(&self) -> &str {
        match self {
            BracersTemplate::BracersOfProtection => "Bracers^ of Protection [%P6,%P4]",
            BracersTemplate::BracersOfDefense => "Bracers^ of Defense [%P6,%P4]",
            BracersTemplate::BracersOfShielding => "Bracers^ of Shielding [%P6,%P4]",
            BracersTemplate::MithrilBracers => "Mithril Bracers^ [%P6,%P4]",
            BracersTemplate::AdamantiteBracers => "Adamantite Bracers^ [%P6,%P4]",
            BracersTemplate::BracersOfWeaponAttraction => "Bracers^ of Weapon Attraction [%P6,%P4]",
            BracersTemplate::SilverBraceletOfWarding => "Silver Bracelet^ of Warding [%P6,%P4] (R)",
            BracersTemplate::SilverBracelet => "Silver Bracelet^ [%P6,%P4]",
            BracersTemplate::GoldBracelet => "Gold Bracelet^ [%P6,%P4]",
            BracersTemplate::PlatinumBracelet => "Platinum Bracelet^ [%P6,%P4]",
            BracersTemplate::LeatherBracers => "Leather Bracers^ [%P6,%P4]",
            BracersTemplate::StuddedLeatherBracers => "Studded Leather Bracers^ [%P6,%P4]",
            BracersTemplate::LightPlatedBracers => "Light Plated Bracers^ [%P6,%P4]",
            BracersTemplate::SharkskinBracers => "Sharkskin Bracers^ [%P6,%P4]",
            BracersTemplate::DemonhideBracers => "Demonhide Bracers^ [%P6,%P4]",
            BracersTemplate::WyrmhideBracers => "Wyrmhide Bracers^ [%P6,%P4]",
            BracersTemplate::ChainmailBracers => "Chainmail Bracers^ [%P6,%P4]",
            BracersTemplate::LamellarBracers => "Lamellar Bracers^ [%P6,%P4]",
        }
    }

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

