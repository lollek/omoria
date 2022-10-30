use crate::model::item_subtype::SlingAmmoSubType;

pub fn from_usize(subtype: usize) -> Option<SlingAmmoSubType> {
    match subtype {
        1 => Some(SlingAmmoSubType::RoundedPebble),
        2 => Some(SlingAmmoSubType::IronShot),
        _ => None,
    }
}

pub fn to_usize(subtype: SlingAmmoSubType) -> usize {
    match subtype {
        SlingAmmoSubType::RoundedPebble => 1,
        SlingAmmoSubType::IronShot => 2,
    }
}
