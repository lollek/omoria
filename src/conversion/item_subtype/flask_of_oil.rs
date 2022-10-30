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
