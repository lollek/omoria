use crate::model::item_subtype::LodgingAtInnSubType;

pub fn from_usize(subtype: usize) -> Option<LodgingAtInnSubType> {
    match subtype {
        300 => Some(LodgingAtInnSubType::LodgingForOneDay),
        302 => Some(LodgingAtInnSubType::LodgingForThreeDays),
        301 => Some(LodgingAtInnSubType::LodgingForOneWeek),
        303 => Some(LodgingAtInnSubType::RoomAndBoardForOneDay),
        _ => None,
    }
}

pub fn to_usize(subtype: LodgingAtInnSubType) -> usize {
    match subtype {
        LodgingAtInnSubType::LodgingForOneDay => 300,
        LodgingAtInnSubType::LodgingForThreeDays => 302,
        LodgingAtInnSubType::LodgingForOneWeek => 301,
        LodgingAtInnSubType::RoomAndBoardForOneDay => 303,
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
