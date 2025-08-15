use crate::model::item_subtype::InstrumentSubType;

pub fn from_usize(subtype: usize) -> Option<InstrumentSubType> {
    match subtype {
        258 => Some(InstrumentSubType::PipesOfPeace),
        259 => Some(InstrumentSubType::LyreOfNature),
        260 => Some(InstrumentSubType::LuteOfTheWoods),
        261 => Some(InstrumentSubType::HarpOfTheDruids),
        _ => None,
    }
}

pub fn to_usize(subtype: &InstrumentSubType) -> usize {
    match subtype {
        InstrumentSubType::PipesOfPeace => 258,
        InstrumentSubType::LyreOfNature => 259,
        InstrumentSubType::LuteOfTheWoods => 260,
        InstrumentSubType::HarpOfTheDruids => 261,
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
