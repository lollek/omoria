use crate::model::item_subtype::SwordSubType;

pub fn from_usize(subtype: usize) -> Option<SwordSubType> {
    match subtype {
        6 => Some(SwordSubType::Backsword),
        7 => Some(SwordSubType::BastardSword),
        10 => Some(SwordSubType::Broadsword),
        11 => Some(SwordSubType::Claymore),
        12 => Some(SwordSubType::Cutlass),
        13 => Some(SwordSubType::Espadon),
        14 => Some(SwordSubType::ExecutionersSword),
        15 => Some(SwordSubType::Flamberge),
        17 => Some(SwordSubType::Katana),
        18 => Some(SwordSubType::Longsword),
        19 => Some(SwordSubType::Nodachi),
        21 => Some(SwordSubType::Sabre),
        23 => Some(SwordSubType::Zweihander),
        24 => Some(SwordSubType::BrokenSword),
        _ => None,
    }
}

pub fn to_usize(subtype: SwordSubType) -> usize {
    match subtype {
        SwordSubType::Backsword => 6,
        SwordSubType::BastardSword => 7,
        SwordSubType::Broadsword => 10,
        SwordSubType::Claymore => 11,
        SwordSubType::Cutlass => 12,
        SwordSubType::Espadon => 13,
        SwordSubType::ExecutionersSword => 14,
        SwordSubType::Flamberge => 15,
        SwordSubType::Katana => 17,
        SwordSubType::Longsword => 18,
        SwordSubType::Nodachi => 19,
        SwordSubType::Sabre => 21,
        SwordSubType::Zweihander => 23,
        SwordSubType::BrokenSword => 24,
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
