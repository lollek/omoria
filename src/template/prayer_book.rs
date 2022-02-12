use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum PrayerBookTemplate {
    BeginnersHandbook,
    WordsOfWisdom,
    ChantsAndBlessings,
    ExorcismAndDispelling,
}

impl PrayerBookTemplate {
    pub fn vec() -> Vec<Box<dyn template::Template>> {
        vec![
            Box::new(PrayerBookTemplate::BeginnersHandbook),
            Box::new(PrayerBookTemplate::WordsOfWisdom),
            Box::new(PrayerBookTemplate::ChantsAndBlessings),
            Box::new(PrayerBookTemplate::ExorcismAndDispelling),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn template::Template>> {
        PrayerBookTemplate::vec().into_iter()
    }
}

impl template::Template for PrayerBookTemplate {
    fn name(&self) -> &str {
        match self {
            PrayerBookTemplate::BeginnersHandbook => "& Holy Book of Prayers [Beginners Handbook]",
            PrayerBookTemplate::WordsOfWisdom => "& Holy Book of Prayers [Words of Wisdom]",
            PrayerBookTemplate::ChantsAndBlessings => "& Holy Book of Prayers [Chants and Blessings]",
            PrayerBookTemplate::ExorcismAndDispelling => "& Holy Book of Prayers [Exorcism and Dispelling]",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::PrayerBook }

    fn flags1(&self) -> u64 {
        match self {
            PrayerBookTemplate::BeginnersHandbook => 0,
            PrayerBookTemplate::WordsOfWisdom => 0,
            PrayerBookTemplate::ChantsAndBlessings => 0x00000001,
            PrayerBookTemplate::ExorcismAndDispelling => 0x000001FE,
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            PrayerBookTemplate::BeginnersHandbook => 0x000000FF,
            PrayerBookTemplate::WordsOfWisdom => 0x0007FF00,
            PrayerBookTemplate::ChantsAndBlessings => 0x7FF80000,
            PrayerBookTemplate::ExorcismAndDispelling => 0,
        }
    }

    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            PrayerBookTemplate::BeginnersHandbook => 25,
            PrayerBookTemplate::WordsOfWisdom => 100,
            PrayerBookTemplate::ChantsAndBlessings => 300,
            PrayerBookTemplate::ExorcismAndDispelling => 900,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            PrayerBookTemplate::BeginnersHandbook => 258,
            PrayerBookTemplate::WordsOfWisdom => 259,
            PrayerBookTemplate::ChantsAndBlessings => 260,
            PrayerBookTemplate::ExorcismAndDispelling => 261,
        }
    }

    fn weight(&self) -> u16 { 60 }
    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }
    fn item_level(&self) -> u8 { 0 }
    fn is_identified(&self) -> bool { false }
}

