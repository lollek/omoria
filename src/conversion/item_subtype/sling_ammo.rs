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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_conversion() {
        (0..1000).for_each(|i| {
            if let Some(subtype) = from_usize(i) {
                assert_eq!(i, to_usize(subtype));
            }
        })
    }
}
