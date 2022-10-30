use crate::model::item_subtype::PoleArmSubType;

pub fn from_usize(subtype: usize) -> Option<PoleArmSubType> {
    match subtype {
        1 => Some(PoleArmSubType::AwlPike),
        2 => Some(PoleArmSubType::BeakedAxe),
        3 => Some(PoleArmSubType::Fauchard),
        4 => Some(PoleArmSubType::Glaive),
        5 => Some(PoleArmSubType::Halberd),
        6 => Some(PoleArmSubType::LucerneHammer),
        7 => Some(PoleArmSubType::Pike),
        8 => Some(PoleArmSubType::Spike),
        9 => Some(PoleArmSubType::Lance),
        10 => Some(PoleArmSubType::Javelin),
        11 => Some(PoleArmSubType::Naginata),
        12 => Some(PoleArmSubType::WarScythe),
        _ => None,
    }
}

pub fn to_usize(subtype: PoleArmSubType) -> usize {
    match subtype {
        PoleArmSubType::AwlPike => 1,
        PoleArmSubType::BeakedAxe => 2,
        PoleArmSubType::Fauchard => 3,
        PoleArmSubType::Glaive => 4,
        PoleArmSubType::Halberd => 5,
        PoleArmSubType::LucerneHammer => 6,
        PoleArmSubType::Pike => 7,
        PoleArmSubType::Spike => 8,
        PoleArmSubType::Lance => 9,
        PoleArmSubType::Javelin => 10,
        PoleArmSubType::Naginata => 11,
        PoleArmSubType::WarScythe => 12,
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
