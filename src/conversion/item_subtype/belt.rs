use crate::model::item_subtype::BeltSubType;

pub fn from_usize(subtype: usize) -> Option<BeltSubType> {
    match subtype {
        1 => Some(BeltSubType::Sash),
        2 => Some(BeltSubType::LightBelt),
        3 => Some(BeltSubType::Belt),
        4 => Some(BeltSubType::HeavyBelt),
        5 => Some(BeltSubType::LightPlatedBelt),
        6 => Some(BeltSubType::SharkskinBelt),
        7 => Some(BeltSubType::DemonhideBelt),
        8 => Some(BeltSubType::WyrmhideBelt),
        _ => None,
    }
}

pub fn to_usize(subtype: BeltSubType) -> usize {
    match subtype {
        BeltSubType::Sash => 1,
        BeltSubType::LightBelt => 2,
        BeltSubType::Belt => 3,
        BeltSubType::HeavyBelt => 4,
        BeltSubType::LightPlatedBelt => 5,
        BeltSubType::SharkskinBelt => 6,
        BeltSubType::DemonhideBelt => 7,
        BeltSubType::WyrmhideBelt => 8,
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
