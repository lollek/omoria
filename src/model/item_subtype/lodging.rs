use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LodgingAtInnSubType {
    LodgingForOneDay,
    LodgingForThreeDays,
    LodgingForOneWeek,
    RoomAndBoardForOneDay,
}

impl From<LodgingAtInnSubType> for usize {
    fn from(value: LodgingAtInnSubType) -> usize {
        match value {
            LodgingAtInnSubType::LodgingForOneDay => 300,
            LodgingAtInnSubType::LodgingForOneWeek => 301,
            LodgingAtInnSubType::LodgingForThreeDays => 302,
            LodgingAtInnSubType::RoomAndBoardForOneDay => 303,
        }
    }
}

impl TryFrom<usize> for LodgingAtInnSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            300 => Ok(LodgingAtInnSubType::LodgingForOneDay),
            301 => Ok(LodgingAtInnSubType::LodgingForOneWeek),
            302 => Ok(LodgingAtInnSubType::LodgingForThreeDays),
            303 => Ok(LodgingAtInnSubType::RoomAndBoardForOneDay),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::LodgingAtInnSubType;

    #[test]
    fn test_lodging_subtype_try_from_usize_accepts_known_values() {
        assert_eq!(
            LodgingAtInnSubType::try_from(300usize).unwrap(),
            LodgingAtInnSubType::LodgingForOneDay
        );
        assert_eq!(
            LodgingAtInnSubType::try_from(301usize).unwrap(),
            LodgingAtInnSubType::LodgingForOneWeek
        );
        assert_eq!(
            LodgingAtInnSubType::try_from(302usize).unwrap(),
            LodgingAtInnSubType::LodgingForThreeDays
        );
        assert_eq!(
            LodgingAtInnSubType::try_from(303usize).unwrap(),
            LodgingAtInnSubType::RoomAndBoardForOneDay
        );
    }

    #[test]
    fn test_lodging_subtype_try_from_usize_rejects_unknown_values() {
        assert!(LodgingAtInnSubType::try_from(299usize).is_err());
        assert!(LodgingAtInnSubType::try_from(304usize).is_err());
    }

    #[test]
    fn test_lodging_subtype_into_usize_returns_expected_codes() {
        let one_day: usize = LodgingAtInnSubType::LodgingForOneDay.into();
        let one_week: usize = LodgingAtInnSubType::LodgingForOneWeek.into();
        let three_days: usize = LodgingAtInnSubType::LodgingForThreeDays.into();
        let room_and_board: usize = LodgingAtInnSubType::RoomAndBoardForOneDay.into();

        assert_eq!(one_day, 300);
        assert_eq!(one_week, 301);
        assert_eq!(three_days, 302);
        assert_eq!(room_and_board, 303);
    }
}
