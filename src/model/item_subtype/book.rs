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

impl From<InstrumentSubType> for usize {
    fn from(value: InstrumentSubType) -> usize {
        match value {
            InstrumentSubType::PipesOfPeace => 258,
            InstrumentSubType::LyreOfNature => 259,
            InstrumentSubType::LuteOfTheWoods => 260,
            InstrumentSubType::HarpOfTheDruids => 261,
        }
    }
}

impl TryFrom<usize> for InstrumentSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            258 => Ok(InstrumentSubType::PipesOfPeace),
            259 => Ok(InstrumentSubType::LyreOfNature),
            260 => Ok(InstrumentSubType::LuteOfTheWoods),
            261 => Ok(InstrumentSubType::HarpOfTheDruids),
            _ => Err(()),
        }
    }
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

    use super::InstrumentSubType;
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

    #[test]
    fn test_instrument_subtype_try_from_usize_accepts_known_values() {
        assert_eq!(
            InstrumentSubType::try_from(258usize).unwrap(),
            InstrumentSubType::PipesOfPeace
        );
        assert_eq!(
            InstrumentSubType::try_from(259usize).unwrap(),
            InstrumentSubType::LyreOfNature
        );
        assert_eq!(
            InstrumentSubType::try_from(260usize).unwrap(),
            InstrumentSubType::LuteOfTheWoods
        );
        assert_eq!(
            InstrumentSubType::try_from(261usize).unwrap(),
            InstrumentSubType::HarpOfTheDruids
        );
    }

    #[test]
    fn test_instrument_subtype_try_from_usize_rejects_unknown_values() {
        assert!(InstrumentSubType::try_from(257usize).is_err());
        assert!(InstrumentSubType::try_from(262usize).is_err());
    }

    #[test]
    fn test_instrument_subtype_into_usize_returns_expected_codes() {
        let pipes: usize = InstrumentSubType::PipesOfPeace.into();
        let lyre: usize = InstrumentSubType::LyreOfNature.into();
        let lute: usize = InstrumentSubType::LuteOfTheWoods.into();
        let harp: usize = InstrumentSubType::HarpOfTheDruids.into();

        assert_eq!(pipes, 258);
        assert_eq!(lyre, 259);
        assert_eq!(lute, 260);
        assert_eq!(harp, 261);
    }
}
