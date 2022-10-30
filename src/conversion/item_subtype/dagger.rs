use crate::model::item_subtype::DaggerSubType;

pub fn from_usize(subtype: usize) -> Option<DaggerSubType> {
    match subtype {
        1 => Some(DaggerSubType::MainGauche),
        2 => Some(DaggerSubType::Misercorde),
        3 => Some(DaggerSubType::Stiletto),
        4 => Some(DaggerSubType::Bodkin),
        5 => Some(DaggerSubType::BrokenDagger),
        6 => Some(DaggerSubType::CatONineTails),
        8 => Some(DaggerSubType::Bilbo),
        9 => Some(DaggerSubType::Baselard),
        16 => Some(DaggerSubType::Foil),
        20 => Some(DaggerSubType::Rapier),
        22 => Some(DaggerSubType::SmallSword),
        _ => None,
    }
}

pub fn to_usize(subtype: DaggerSubType) -> usize {
    match subtype {
        DaggerSubType::MainGauche => 1,
        DaggerSubType::Misercorde => 2,
        DaggerSubType::Stiletto => 3,
        DaggerSubType::Bodkin => 4,
        DaggerSubType::BrokenDagger => 5,
        DaggerSubType::CatONineTails => 6,
        DaggerSubType::Bilbo => 8,
        DaggerSubType::Baselard => 9,
        DaggerSubType::Foil => 16,
        DaggerSubType::Rapier => 20,
        DaggerSubType::SmallSword => 22,
    }
}
