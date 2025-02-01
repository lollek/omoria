use crate::model::item_subtype::ShieldSubType;

pub fn from_usize(subtype: usize) -> Option<ShieldSubType> {
    match subtype {
        1 => Some(ShieldSubType::SmallLeatherShield),
        2 => Some(ShieldSubType::MediumLeatherShield),
        3 => Some(ShieldSubType::LargeLeatherShield),
        4 => Some(ShieldSubType::Buckler),
        5 => Some(ShieldSubType::KiteShield),
        6 => Some(ShieldSubType::TowerShield),
        7 => Some(ShieldSubType::SharkskinShield),
        8 => Some(ShieldSubType::DemonhideShield),
        9 => Some(ShieldSubType::WyrmhideShield),
        _ => None,
    }
}

pub fn to_usize(subtype: ShieldSubType) -> usize {
    match subtype {
        ShieldSubType::SmallLeatherShield => 1,
        ShieldSubType::MediumLeatherShield => 2,
        ShieldSubType::LargeLeatherShield => 3,
        ShieldSubType::Buckler => 4,
        ShieldSubType::KiteShield => 5,
        ShieldSubType::TowerShield => 6,
        ShieldSubType::SharkskinShield => 7,
        ShieldSubType::DemonhideShield => 8,
        ShieldSubType::WyrmhideShield => 9,
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
