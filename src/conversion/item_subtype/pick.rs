use crate::model::item_subtype::PickSubType;

pub fn from_usize(subtype: usize) -> Option<PickSubType> {
    match subtype {
        1 => Some(PickSubType::Pick),
        2 => Some(PickSubType::Shovel),
        8 => Some(PickSubType::OrcishPick1),
        7 => Some(PickSubType::OrcishPick2),
        3 => Some(PickSubType::DwarvenPick),
        5 => Some(PickSubType::GnomishShovel),
        6 => Some(PickSubType::DwarvenShovel),
        _ => None,
    }
}

pub fn to_usize(subtype: PickSubType) -> usize {
    match subtype {
        PickSubType::Pick => 1,
        PickSubType::Shovel => 2,
        PickSubType::OrcishPick1 => 8,
        PickSubType::OrcishPick2 => 7,
        PickSubType::DwarvenPick => 3,
        PickSubType::GnomishShovel => 5,
        PickSubType::DwarvenShovel => 6,
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
