use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum PrayerBookTemplate {
    BeginnersHandbook,
    WordsOfWisdom,
    ChantsAndBlessings,
    ExorcismAndDispelling,
}

impl PrayerBookTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(PrayerBookTemplate::BeginnersHandbook),
            Box::new(PrayerBookTemplate::WordsOfWisdom),
            Box::new(PrayerBookTemplate::ChantsAndBlessings),
            Box::new(PrayerBookTemplate::ExorcismAndDispelling),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        PrayerBookTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            258 => Box::new(PrayerBookTemplate::BeginnersHandbook),
            259 => Box::new(PrayerBookTemplate::WordsOfWisdom),
            260 => Box::new(PrayerBookTemplate::ChantsAndBlessings),
            261 => Box::new(PrayerBookTemplate::ExorcismAndDispelling),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for PrayerBookTemplate {
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

