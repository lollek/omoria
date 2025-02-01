use crate::model::item_subtype::SpikeSubType;

pub fn from_usize(subtype: usize) -> Option<SpikeSubType> {
    match subtype {
        1 => Some(SpikeSubType::IronSpike),
        _ => None,
    }
}

pub fn to_usize(subtype: SpikeSubType) -> usize {
    match subtype {
        SpikeSubType::IronSpike => 1,
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
