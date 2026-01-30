use crate::model::item_subtype::GemHelmSubType;

pub fn from_usize(subtype: usize) -> Option<GemHelmSubType> {
    match subtype {
        9 => Some(GemHelmSubType::IronHelm),
        10 => Some(GemHelmSubType::SteelHelm),
        _ => None,
    }
}

pub fn to_usize(subtype: &GemHelmSubType) -> usize {
    match subtype {
        GemHelmSubType::IronHelm => 9,
        GemHelmSubType::SteelHelm => 10,
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
