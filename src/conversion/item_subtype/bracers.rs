use crate::model::item_subtype::BracersSubType;

pub fn from_usize(subtype: usize) -> Option<BracersSubType> {
    match subtype {
        1 => Some(BracersSubType::BracersOfProtection),
        2 => Some(BracersSubType::BracersOfDefense),
        3 => Some(BracersSubType::BracersOfShielding),
        4 => Some(BracersSubType::MithrilBracers),
        5 => Some(BracersSubType::AdamantiteBracers),
        6 => Some(BracersSubType::BracersOfWeaponAttraction),
        31 => Some(BracersSubType::SilverBraceletOfWarding),
        30 => Some(BracersSubType::SilverBracelet),
        40 => Some(BracersSubType::GoldBracelet),
        50 => Some(BracersSubType::PlatinumBracelet),
        7 => Some(BracersSubType::LeatherBracers),
        8 => Some(BracersSubType::StuddedLeatherBracers),
        9 => Some(BracersSubType::LightPlatedBracers),
        10 => Some(BracersSubType::SharkskinBracers),
        11 => Some(BracersSubType::DemonhideBracers),
        12 => Some(BracersSubType::WyrmhideBracers),
        13 => Some(BracersSubType::ChainmailBracers),
        14 => Some(BracersSubType::LamellarBracers),
        _ => None,
    }
}

pub fn to_usize(subtype: BracersSubType) -> usize {
    match subtype {
        BracersSubType::BracersOfProtection => 1,
        BracersSubType::BracersOfDefense => 2,
        BracersSubType::BracersOfShielding => 3,
        BracersSubType::MithrilBracers => 4,
        BracersSubType::AdamantiteBracers => 5,
        BracersSubType::BracersOfWeaponAttraction => 6,
        BracersSubType::SilverBraceletOfWarding => 31,
        BracersSubType::SilverBracelet => 30,
        BracersSubType::GoldBracelet => 40,
        BracersSubType::PlatinumBracelet => 50,
        BracersSubType::LeatherBracers => 7,
        BracersSubType::StuddedLeatherBracers => 8,
        BracersSubType::LightPlatedBracers => 9,
        BracersSubType::SharkskinBracers => 10,
        BracersSubType::DemonhideBracers => 11,
        BracersSubType::WyrmhideBracers => 12,
        BracersSubType::ChainmailBracers => 13,
        BracersSubType::LamellarBracers => 14,
    }
}
