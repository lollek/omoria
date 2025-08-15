use crate::model::item_subtype::LightSourceSubType;

pub fn from_usize(subtype: usize) -> Option<LightSourceSubType> {
    match subtype {
        1 => Some(LightSourceSubType::BrassLantern),
        13 => Some(LightSourceSubType::WoodenTorch),
        30 => Some(LightSourceSubType::MagicTorch),
        17 => Some(LightSourceSubType::MagicLantern),
        _ => None,
    }
}

pub fn to_usize(light_source: &LightSourceSubType) -> usize {
    match light_source {
        LightSourceSubType::WoodenTorch => 13,
        LightSourceSubType::BrassLantern => 1,
        LightSourceSubType::MagicTorch => 30,
        LightSourceSubType::MagicLantern => 17,
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
