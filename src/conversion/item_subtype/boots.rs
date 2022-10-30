use crate::model::item_subtype::BootsSubType;

pub fn from_usize(subtype: usize) -> Option<BootsSubType> {
    match subtype {
        1 => Some(BootsSubType::SoftLeatherShoes),
        2 => Some(BootsSubType::SoftLeatherBoots),
        3 => Some(BootsSubType::HardLeatherBoots),
        4 => Some(BootsSubType::Sandals),
        5 => Some(BootsSubType::ChainBoots),
        6 => Some(BootsSubType::LightPlatedBoots),
        7 => Some(BootsSubType::SharkskinBoots),
        8 => Some(BootsSubType::DemonhideBoots),
        9 => Some(BootsSubType::WyrmhideBoot),
        _ => None,
    }
}

pub fn to_usize(subtype: BootsSubType) -> usize {
    match subtype {
        BootsSubType::SoftLeatherShoes => 1,
        BootsSubType::SoftLeatherBoots => 2,
        BootsSubType::HardLeatherBoots => 3,
        BootsSubType::Sandals => 4,
        BootsSubType::ChainBoots => 5,
        BootsSubType::LightPlatedBoots => 6,
        BootsSubType::SharkskinBoots => 7,
        BootsSubType::DemonhideBoots => 8,
        BootsSubType::WyrmhideBoot => 9,
    }
}

#[cfg(test)]
mod test {
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
