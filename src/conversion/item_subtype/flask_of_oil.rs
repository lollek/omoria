use crate::model::item_subtype::FlaskOfOilSubType;

pub fn from_usize(subtype: usize) -> Option<FlaskOfOilSubType> {
    match subtype {
        1 => Some(FlaskOfOilSubType::FlaskOfOil),
        _ => None,
    }
}

pub fn to_usize(subtype: FlaskOfOilSubType) -> usize {
    match subtype {
        FlaskOfOilSubType::FlaskOfOil => 1,
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
