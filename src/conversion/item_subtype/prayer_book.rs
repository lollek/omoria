use crate::model::item_subtype::PrayerBookSubType;

pub fn from_usize(subtype: usize) -> Option<PrayerBookSubType> {
    match subtype {
        258 => Some(PrayerBookSubType::BeginnersHandbook),
        259 => Some(PrayerBookSubType::WordsOfWisdom),
        260 => Some(PrayerBookSubType::ChantsAndBlessings),
        261 => Some(PrayerBookSubType::ExorcismAndDispelling),
        _ => None,
    }
}

pub fn to_usize(subtype: PrayerBookSubType) -> usize {
    match subtype {
        PrayerBookSubType::BeginnersHandbook => 258,
        PrayerBookSubType::WordsOfWisdom => 259,
        PrayerBookSubType::ChantsAndBlessings => 260,
        PrayerBookSubType::ExorcismAndDispelling => 261,
    }
}
