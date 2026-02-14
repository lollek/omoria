#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LodgingAtInnSubType {
    LodgingForOneDay,
    LodgingForThreeDays,
    LodgingForOneWeek,
    RoomAndBoardForOneDay,
}
