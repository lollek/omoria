use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MagicBookSubType {
    BeginnersMagic,
    Magic1,
    Magic2,
    MagesGuideToPower,
}

impl From<MagicBookSubType> for usize {
    fn from(value: MagicBookSubType) -> usize {
        match value {
            MagicBookSubType::BeginnersMagic => 257,
            MagicBookSubType::Magic1 => 258,
            MagicBookSubType::Magic2 => 259,
            MagicBookSubType::MagesGuideToPower => 261,
        }
    }
}

impl TryFrom<usize> for MagicBookSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            257 => Ok(MagicBookSubType::BeginnersMagic),
            258 => Ok(MagicBookSubType::Magic1),
            259 => Ok(MagicBookSubType::Magic2),
            261 => Ok(MagicBookSubType::MagesGuideToPower),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrayerBookSubType {
    BeginnersHandbook,
    WordsOfWisdom,
    ChantsAndBlessings,
    ExorcismAndDispelling,
}

impl From<PrayerBookSubType> for usize {
    fn from(value: PrayerBookSubType) -> usize {
        match value {
            PrayerBookSubType::BeginnersHandbook => 258,
            PrayerBookSubType::WordsOfWisdom => 259,
            PrayerBookSubType::ChantsAndBlessings => 260,
            PrayerBookSubType::ExorcismAndDispelling => 261,
        }
    }
}

impl TryFrom<usize> for PrayerBookSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            258 => Ok(PrayerBookSubType::BeginnersHandbook),
            259 => Ok(PrayerBookSubType::WordsOfWisdom),
            260 => Ok(PrayerBookSubType::ChantsAndBlessings),
            261 => Ok(PrayerBookSubType::ExorcismAndDispelling),
            _ => Err(()),
        }
    }
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
    use super::MagicBookSubType;
    use super::PrayerBookSubType;
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

    #[test]
    fn test_prayer_book_subtype_try_from_usize_accepts_known_values() {
        assert_eq!(
            PrayerBookSubType::try_from(258usize).unwrap(),
            PrayerBookSubType::BeginnersHandbook
        );
        assert_eq!(
            PrayerBookSubType::try_from(259usize).unwrap(),
            PrayerBookSubType::WordsOfWisdom
        );
        assert_eq!(
            PrayerBookSubType::try_from(260usize).unwrap(),
            PrayerBookSubType::ChantsAndBlessings
        );
        assert_eq!(
            PrayerBookSubType::try_from(261usize).unwrap(),
            PrayerBookSubType::ExorcismAndDispelling
        );
    }

    #[test]
    fn test_prayer_book_subtype_try_from_usize_rejects_unknown_values() {
        assert!(PrayerBookSubType::try_from(257usize).is_err());
        assert!(PrayerBookSubType::try_from(262usize).is_err());
    }

    #[test]
    fn test_prayer_book_subtype_into_usize_returns_expected_codes() {
        let beginners: usize = PrayerBookSubType::BeginnersHandbook.into();
        let wisdom: usize = PrayerBookSubType::WordsOfWisdom.into();
        let chants: usize = PrayerBookSubType::ChantsAndBlessings.into();
        let exorcism: usize = PrayerBookSubType::ExorcismAndDispelling.into();

        assert_eq!(beginners, 258);
        assert_eq!(wisdom, 259);
        assert_eq!(chants, 260);
        assert_eq!(exorcism, 261);
    }

    #[test]
    fn test_magic_book_subtype_try_from_usize_accepts_known_values() {
        assert_eq!(
            MagicBookSubType::try_from(257usize).unwrap(),
            MagicBookSubType::BeginnersMagic
        );
        assert_eq!(
            MagicBookSubType::try_from(258usize).unwrap(),
            MagicBookSubType::Magic1
        );
        assert_eq!(
            MagicBookSubType::try_from(259usize).unwrap(),
            MagicBookSubType::Magic2
        );
        assert_eq!(
            MagicBookSubType::try_from(261usize).unwrap(),
            MagicBookSubType::MagesGuideToPower
        );
    }

    #[test]
    fn test_magic_book_subtype_try_from_usize_rejects_unknown_values() {
        assert!(MagicBookSubType::try_from(256usize).is_err());
        assert!(MagicBookSubType::try_from(260usize).is_err());
        assert!(MagicBookSubType::try_from(262usize).is_err());
    }

    #[test]
    fn test_magic_book_subtype_into_usize_returns_expected_codes() {
        let beginners: usize = MagicBookSubType::BeginnersMagic.into();
        let magic_1: usize = MagicBookSubType::Magic1.into();
        let magic_2: usize = MagicBookSubType::Magic2.into();
        let guide: usize = MagicBookSubType::MagesGuideToPower.into();

        assert_eq!(beginners, 257);
        assert_eq!(magic_1, 258);
        assert_eq!(magic_2, 259);
        assert_eq!(guide, 261);
    }
}
