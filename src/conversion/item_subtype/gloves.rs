use crate::model::item_subtype::GlovesSubType;

pub fn from_usize(subtype: usize) -> Option<GlovesSubType> {
    match subtype {
        1 => Some(GlovesSubType::LeatherGloves),
        2 => Some(GlovesSubType::HeavyGloves),
        5 => Some(GlovesSubType::ClothGloves),
        6 => Some(GlovesSubType::ChainGloves),
        7 => Some(GlovesSubType::LightGauntlets),
        8 => Some(GlovesSubType::HeavyGauntlets),
        9 => Some(GlovesSubType::SharkskinGloves),
        10 => Some(GlovesSubType::WarGauntlets),
        11 => Some(GlovesSubType::DemonhideGloves),
        12 => Some(GlovesSubType::WyrmhideGloves),
        _ => None,
    }
}

pub fn to_usize(subtype: &GlovesSubType) -> usize {
    match subtype {
        GlovesSubType::LeatherGloves => 1,
        GlovesSubType::HeavyGloves => 2,
        GlovesSubType::ClothGloves => 5,
        GlovesSubType::ChainGloves => 6,
        GlovesSubType::LightGauntlets => 7,
        GlovesSubType::HeavyGauntlets => 8,
        GlovesSubType::SharkskinGloves => 9,
        GlovesSubType::WarGauntlets => 10,
        GlovesSubType::DemonhideGloves => 11,
        GlovesSubType::WyrmhideGloves => 12,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_conversion() {
        (0..1000).for_each(|i| {
            if let Some(subtype) = from_usize(i) {
                assert_eq!(i, to_usize(&subtype));
            }
        })
    }
}
