use misc;
use model;

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
    pub fn iter() -> impl Iterator<Item=BracersTemplate> {
        [
            BracersTemplate::BracersOfProtection,
            BracersTemplate::BracersOfDefense,
            BracersTemplate::BracersOfShielding,
            BracersTemplate::MithrilBracers,
            BracersTemplate::AdamantiteBracers,
            BracersTemplate::BracersOfWeaponAttraction,
            BracersTemplate::SilverBraceletOfWarding,
            BracersTemplate::SilverBracelet,
            BracersTemplate::GoldBracelet,
            BracersTemplate::PlatinumBracelet,
            BracersTemplate::LeatherBracers,
            BracersTemplate::StuddedLeatherBracers,
            BracersTemplate::LightPlatedBracers,
            BracersTemplate::SharkskinBracers,
            BracersTemplate::DemonhideBracers,
            BracersTemplate::WyrmhideBracers,
            BracersTemplate::ChainmailBracers,
            BracersTemplate::LamellarBracers,
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

    fn subval(&self) -> i64 {
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

    fn ac(&self) -> i16 {
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
    fn leve(&self) -> u8 {
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

}

