use crate::model::item_subtype::HelmSubType;

pub fn from_usize(subtype: usize) -> Option<HelmSubType> {
    match subtype {
        12 => Some(HelmSubType::ClothHat),
        13 => Some(HelmSubType::SoftLeatherCap),
        14 => Some(HelmSubType::HardLeatherCap),
        15 => Some(HelmSubType::MetalCap),
        16 => Some(HelmSubType::FullHelm),
        17 => Some(HelmSubType::GreatHelm),
        18 => Some(HelmSubType::WingedHelm),
        19 => Some(HelmSubType::SilverCrown),
        20 => Some(HelmSubType::SilverMask),
        21 => Some(HelmSubType::GoldenCrown),
        22 => Some(HelmSubType::GoldenMask),
        23 => Some(HelmSubType::JewelEncrustedCrown),
        _ => None,
    }
}

pub fn to_usize(subtype: HelmSubType) -> usize {
    match subtype {
        HelmSubType::ClothHat => 12,
        HelmSubType::SoftLeatherCap => 13,
        HelmSubType::HardLeatherCap => 14,
        HelmSubType::MetalCap => 15,
        HelmSubType::FullHelm => 16,
        HelmSubType::GreatHelm => 17,
        HelmSubType::WingedHelm => 18,
        HelmSubType::SilverCrown => 19,
        HelmSubType::SilverMask => 20,
        HelmSubType::GoldenCrown => 21,
        HelmSubType::GoldenMask => 22,
        HelmSubType::JewelEncrustedCrown => 23,
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
