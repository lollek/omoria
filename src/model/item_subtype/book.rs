#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MagicBookSubType {
    BeginnersMagic,
    Magic1,
    Magic2,
    MagesGuideToPower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrayerBookSubType {
    BeginnersHandbook,
    WordsOfWisdom,
    ChantsAndBlessings,
    ExorcismAndDispelling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstrumentSubType {
    PipesOfPeace,
    LyreOfNature,
    LuteOfTheWoods,
    HarpOfTheDruids,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SongBookSubType {
    BeginnersHandbook,
    SongBook1,
    SongBook2,
    GreaterSongBook,
}
