use std::convert::TryFrom;

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

impl From<SongBookSubType> for usize {
    fn from(value: SongBookSubType) -> usize {
        match value {
            SongBookSubType::BeginnersHandbook => 262,
            SongBookSubType::SongBook1 => 263,
            SongBookSubType::SongBook2 => 264,
            SongBookSubType::GreaterSongBook => 265,
        }
    }
}

impl TryFrom<usize> for SongBookSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            262 => Ok(SongBookSubType::BeginnersHandbook),
            263 => Ok(SongBookSubType::SongBook1),
            264 => Ok(SongBookSubType::SongBook2),
            265 => Ok(SongBookSubType::GreaterSongBook),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::SongBookSubType;

    #[test]
    fn test_song_book_subtype_try_from_usize_accepts_known_values() {
        assert_eq!(
            SongBookSubType::try_from(262usize).unwrap(),
            SongBookSubType::BeginnersHandbook
        );
        assert_eq!(
            SongBookSubType::try_from(263usize).unwrap(),
            SongBookSubType::SongBook1
        );
        assert_eq!(
            SongBookSubType::try_from(264usize).unwrap(),
            SongBookSubType::SongBook2
        );
        assert_eq!(
            SongBookSubType::try_from(265usize).unwrap(),
            SongBookSubType::GreaterSongBook
        );
    }

    #[test]
    fn test_song_book_subtype_try_from_usize_rejects_unknown_values() {
        assert!(SongBookSubType::try_from(261usize).is_err());
        assert!(SongBookSubType::try_from(266usize).is_err());
    }

    #[test]
    fn test_song_book_subtype_into_usize_returns_expected_codes() {
        let beginners: usize = SongBookSubType::BeginnersHandbook.into();
        let book_1: usize = SongBookSubType::SongBook1.into();
        let book_2: usize = SongBookSubType::SongBook2.into();
        let greater: usize = SongBookSubType::GreaterSongBook.into();

        assert_eq!(beginners, 262);
        assert_eq!(book_1, 263);
        assert_eq!(book_2, 264);
        assert_eq!(greater, 265);
    }
}
